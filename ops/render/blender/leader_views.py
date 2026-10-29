"""Blender scene: studio renders of one design STL, cameras framed automatically from the mesh.

    blender -b --factory-startup -P ops/render/blender/leader_views.py -- MESH.stl OUT_DIR \
        [--views hero,reverse,...] [--samples 64] [--res 1600x1000] [--threads 8]

Frame-agnostic (any CAD frame): the long axis is X, the tip is the vertex of max X, the ring is the
min-X end. Hand zone (X below the grip/blade split) is anodized Ti, the blade bare satin Ti.
Section views cut the actual mesh with bmesh bisect + fill; nothing is re-solved or re-scored.
"""
import math
import os
import sys

import bmesh
import bpy
from mathutils import Vector

argv = sys.argv[sys.argv.index("--") + 1:]
mesh_path, out_dir = argv[0], argv[1]
opts = {"views": "hero,reverse,side,top,ring,grip,blade,tip,tip_edge,cut_full,cut_x,cut_b",
        "samples": "64", "res": "1600x1000", "threads": "8"}
i = 2
while i < len(argv):
    opts[argv[i][2:]] = argv[i + 1]
    i += 2
os.makedirs(out_dir, exist_ok=True)

bpy.ops.wm.read_factory_settings(use_empty=True)
sc = bpy.context.scene
sc.render.engine = "CYCLES"
sc.cycles.device = "CPU"
sc.cycles.samples = int(opts["samples"])
sc.cycles.use_denoising = True
sc.render.threads_mode = "FIXED"
sc.render.threads = int(opts["threads"])
w, h = (int(v) for v in opts["res"].split("x"))
sc.render.resolution_x, sc.render.resolution_y = w, h
sc.render.image_settings.file_format = "PNG"
sc.view_settings.view_transform = "AgX"
world = bpy.data.worlds.new("w")
sc.world = world
world.use_nodes = True
bg = world.node_tree.nodes["Background"]
bg.inputs[0].default_value = (0.012, 0.013, 0.016, 1)
bg.inputs[1].default_value = 1.0

bpy.ops.wm.stl_import(filepath=mesh_path)
src = bpy.context.selected_objects[0]
src.scale = (0.001, 0.001, 0.001)
bpy.context.view_layer.objects.active = src
bpy.ops.object.transform_apply(scale=True)
bm = bmesh.new()
bm.from_mesh(src.data)
bmesh.ops.remove_doubles(bm, verts=bm.verts, dist=1e-7)
bm.to_mesh(src.data)
bm.free()
for p in src.data.polygons:
    p.use_smooth = True
mod = src.modifiers.new("ws", "WEIGHTED_NORMAL")
mod.keep_sharp = True

V = [v.co.copy() for v in src.data.vertices]
xs = [v.x for v in V]
X0, X1 = min(xs), max(xs)
L = X1 - X0
tip = max(V, key=lambda v: v.x)
split_x = X0 + 0.47 * L  # hand zone / blade split (illustrative finish only)


def region(lo, hi):
    pts = [v for v in V if lo <= v.x <= hi]
    mn = Vector((min(p.x for p in pts), min(p.y for p in pts), min(p.z for p in pts)))
    mx = Vector((max(p.x for p in pts), max(p.y for p in pts), max(p.z for p in pts)))
    return (mn + mx) / 2, (mx - mn).length / 2


def mat_metal(name, base, rough, coat=0.0):
    m = bpy.data.materials.new(name)
    m.use_nodes = True
    b = m.node_tree.nodes["Principled BSDF"]
    b.inputs["Base Color"].default_value = (*base, 1)
    b.inputs["Metallic"].default_value = 1.0
    b.inputs["Roughness"].default_value = rough
    b.inputs["Coat Weight"].default_value = coat
    return m


HANDLE = mat_metal("anod", (0.10, 0.20, 0.55), 0.30, 0.4)
BLADE = mat_metal("satin", (0.78, 0.77, 0.75), 0.22)
CUTFACE = mat_metal("cutface", (0.85, 0.45, 0.20), 0.35)
src.data.materials.append(HANDLE)
src.data.materials.append(BLADE)
for p in src.data.polygons:
    p.material_index = 0 if p.center.x < split_x else 1

floor_z = min(v.z for v in V) - 0.0005
bpy.ops.mesh.primitive_plane_add(size=2.0, location=((X0 + X1) / 2, 0, floor_z))
floor = bpy.context.active_object
fm = bpy.data.materials.new("floor")
fm.use_nodes = True
fm.node_tree.nodes["Principled BSDF"].inputs["Base Color"].default_value = (0.004, 0.0045, 0.005, 1)
fm.node_tree.nodes["Principled BSDF"].inputs["Roughness"].default_value = 0.6
floor.data.materials.append(fm)


def light(name, loc, energy, size, color=(1, 1, 1)):
    ld = bpy.data.lights.new(name, "AREA")
    ld.energy, ld.size, ld.color = energy, size, color
    ob = bpy.data.objects.new(name, ld)
    sc.collection.objects.link(ob)
    ob.location = loc
    d = Vector(((X0 + X1) / 2, 0, 0)) - Vector(loc)
    ob.rotation_euler = d.to_track_quat("-Z", "Y").to_euler()


light("key", (0.05, -0.25, 0.35), 18, 0.35)
light("fill", (0.25, 0.25, 0.20), 6, 0.5, (0.8, 0.9, 1.0))
light("rim", (-0.2, 0.15, 0.10), 10, 0.25, (1.0, 0.9, 0.8))

cam_data = bpy.data.cameras.new("cam")
cam = bpy.data.objects.new("cam", cam_data)
sc.collection.objects.link(cam)
sc.camera = cam


def frame(center, radius, direction, lens=70, roll_up=(0, 0, 1)):
    cam_data.lens = lens
    hfov = 2 * math.atan(cam_data.sensor_width / (2 * lens))
    dist = radius * 1.1 / math.tan(hfov / 2)
    d = Vector(direction).normalized()
    cam.location = center + d * dist
    cam.rotation_euler = (center - cam.location).to_track_quat("-Z", "Y" if abs(d.z) < 0.99 else "X").to_euler()
    cam_data.clip_start, cam_data.clip_end = 0.001, 10


def section(normal, point, keep_positive=True):
    """Copy of the design cut by a plane; the cut face is filled and coloured."""
    ob = src.copy()
    ob.data = src.data.copy()
    ob.modifiers.clear()
    sc.collection.objects.link(ob)
    bm = bmesh.new()
    bm.from_mesh(ob.data)
    n = Vector(normal) * (-1 if keep_positive else 1)
    geom = bm.verts[:] + bm.edges[:] + bm.faces[:]
    res = bmesh.ops.bisect_plane(bm, geom=geom, plane_co=Vector(point), plane_no=n,
                                 clear_inner=False, clear_outer=True, dist=1e-7)
    cut_edges = [e for e in res["geom_cut"] if isinstance(e, bmesh.types.BMEdge)]
    if cut_edges:
        filled = bmesh.ops.holes_fill(bm, edges=cut_edges, sides=0)
        ob.data.materials.append(CUTFACE)
        for f in filled["faces"]:
            f.material_index = len(ob.data.materials) - 1
            f.smooth = False
    bm.to_mesh(ob.data)
    bm.free()
    return ob


c_all, r_all = region(X0, X1)
c_ring, r_ring = region(X0, X0 + 0.28 * L)
c_grip, r_grip = region(X0 + 0.2 * L, split_x + 0.05 * L)
c_blade, r_blade = region(split_x, X1)
c_tip, r_tip = region(X1 - 0.16 * L, X1)
x_grip_cut = X0 + 0.36 * L
x_blade_cut = X1 - 0.18 * L

views = {
    "hero": lambda: frame(c_all, r_all, (0.25, -0.7, 0.75), 70),
    "reverse": lambda: frame(c_all, r_all, (-0.25, 0.7, 0.75), 70),
    "side": lambda: frame(c_all, r_all, (0, 0, 1), 85),
    "top": lambda: frame(c_all, r_all * 0.75, (0, -1, 0.04), 85),
    "ring": lambda: frame(c_ring, r_ring, (-0.3, -0.5, 0.8), 85),
    "grip": lambda: frame(c_grip, r_grip, (0.1, -0.6, 0.8), 85),
    "blade": lambda: frame(c_blade, r_blade, (0.2, -0.6, 0.8), 70),
    "tip": lambda: frame(c_tip, r_tip, (0.3, -0.4, 0.85), 85),
    "tip_edge": lambda: frame(c_tip, r_tip, (0.8, -0.3, 0.35), 85),
}
sections = {
    "cut_full": ((0, 0, 1), (0, 0, 0), lambda: frame(c_all, r_all, (0.15, -0.55, 0.85), 70)),
    "cut_x": ((1, 0, 0), (x_grip_cut, 0, 0), lambda: frame(region(x_grip_cut - 3e-3, x_grip_cut + 3e-3)[0],
                                                             region(x_grip_cut - 3e-3, x_grip_cut + 3e-3)[1] * 1.6,
                                                             (-1, -0.35, 0.35), 85)),
    "cut_b": ((1, 0, 0), (x_blade_cut, 0, 0), lambda: frame(region(x_blade_cut - 3e-3, x_blade_cut + 3e-3)[0],
                                                              region(x_blade_cut - 3e-3, x_blade_cut + 3e-3)[1] * 1.6,
                                                              (-1, -0.35, 0.35), 85)),
}
# cut_x / cut_b keep the +X side (toward the tip) and look at the face from -X
for v in opts["views"].split(","):
    extra = None
    if v in sections:
        n, p, cam_fn = sections[v]
        extra = section(n, p, keep_positive=(v != "cut_full"))
        src.hide_render = True
        cam_fn()
    elif v in views:
        src.hide_render = False
        views[v]()
    else:
        print("unknown view", v)
        continue
    sc.render.filepath = os.path.join(out_dir, v + ".png")
    bpy.ops.render.render(write_still=True)
    print("WROTE", sc.render.filepath, flush=True)
    if extra is not None:
        bpy.data.objects.remove(extra, do_unlink=True)
    src.hide_render = False
