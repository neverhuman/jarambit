"""Blender adapter for the retained paper scene library; no CAD is regenerated.

The host wrapper validates the context. Typography is applied by Pillow later;
the adapter replaces the library's fixed #84 status with this assessment's status.
"""
import json
import os
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import paper_scenes as atlas

import argparse
import hashlib
ROOT=atlas.ROOT
SUBJECT=atlas.SUBJECT
STL=atlas.STL
bpy=atlas.bpy
np=atlas.np
math=atlas.math
Vector=atlas.Vector
material=atlas.material
mesh_obj=atlas.mesh_obj
stl_triangles=atlas.stl_triangles
light=atlas.light

def publication_beauty(args):
    """Presentation-only studio image; the imported assessed mesh is immutable."""
    spec=json.loads((ROOT/'docs/paper/figure-inputs.v1.json').read_text())
    if spec['subject'] != SUBJECT or spec['geometry']['path'] != str(STL.relative_to(ROOT)):
        raise ValueError('publication geometry identity differs')
    for path,digest in spec['inputs'].items():
        if hashlib.sha256((ROOT/path).read_bytes()).hexdigest() != digest:
            raise ValueError('publication input changed: '+path)
    bpy.ops.object.select_all(action='SELECT');bpy.ops.object.delete(use_global=False)
    s=bpy.context.scene;s.render.engine='CYCLES';s.cycles.device='CPU'
    s.cycles.samples=args.samples;s.cycles.seed=42;s.cycles.use_denoising=True
    s.render.threads_mode='FIXED';s.render.threads=4
    s.render.resolution_x=args.width;s.render.resolution_y=args.height;s.render.resolution_percentage=100
    s.render.image_settings.file_format='PNG';s.render.image_settings.color_mode='RGB'
    s.render.image_settings.color_depth='8';s.render.image_settings.compression=85
    s.render.use_compositing=False;s.view_settings.view_transform='AgX'
    s.view_settings.exposure=0;s.view_settings.gamma=1
    s.unit_settings.system='METRIC';s.unit_settings.length_unit='MILLIMETERS'
    styles={
        'neutral': {'floor':(.055,.073,.09),'world':.22,'roughness':.37},
        'dark': {'floor':(.008,.014,.022),'world':.12,'roughness':.32},
        'editorial': {'floor':(.69,.71,.70),'world':.35,'roughness':.4},
    }
    style=styles[args.recipe]
    s.world.use_nodes=True
    s.world.node_tree.nodes['Background'].inputs['Color'].default_value=(.72,.8,.9,1)
    s.world.node_tree.nodes['Background'].inputs['Strength'].default_value=style['world']
    tri=stl_triangles(STL)*.001
    vertices,indices=np.unique(tri.reshape(-1,3),axis=0,return_inverse=True)
    ti=material('Illustrative neutral titanium',(.32,.36,.41),.82,style['roughness'])
    ob=mesh_obj('Exact assessed export / '+SUBJECT,vertices.tolist(),indices.reshape(-1,3).tolist(),ti)
    before=np.asarray([v.co[:] for v in ob.data.vertices],dtype='<f4')
    faces=np.asarray([p.vertices[:] for p in ob.data.polygons],dtype='<u4')
    center=(vertices.min(axis=0)+vertices.max(axis=0))/2
    target=Vector(center)
    floor=material('Studio background',style['floor'],0,.52)
    bpy.ops.mesh.primitive_plane_add(size=2,location=(target.x,target.y,float(vertices[:,2].min())-.0015))
    bpy.context.object.data.materials.append(floor)
    bpy.ops.object.camera_add(location=target+Vector((.015,-.16,.46)))
    cam=bpy.context.object;cam.name='Publication camera'
    cam.rotation_euler=(target-cam.location).to_track_quat('-Z','Y').to_euler()
    cam.rotation_euler.rotate_axis('Z',math.radians(28))
    cam.data.type='ORTHO';cam.data.ortho_scale=.25;cam.data.dof.use_dof=False;s.camera=cam
    # Fit the actual projected bounding box with 9% margin; do not transform the mesh.
    from bpy_extras.object_utils import world_to_camera_view
    bpy.context.view_layer.update()
    for _ in range(2):
        projected=[world_to_camera_view(s,cam,ob.matrix_world@Vector(v)) for v in ob.bound_box]
        extent=max(max(abs(q.x-.5),abs(q.y-.5)) for q in projected)*2
        cam.data.ortho_scale*=extent/.82
        bpy.context.view_layer.update()
    light('Broad key',target+Vector((-.06,-.12,.3)),1.7,.25,(.88,.94,1),target)
    light('Warm edge',target+Vector((.11,.13,.16)),1.4,.18,(1,.88,.74),target)
    light('Soft fill',target+Vector((.14,-.04,.14)),.65,.2,(.78,.88,1),target)
    after=np.asarray([v.co[:] for v in ob.data.vertices],dtype='<f4')
    after_faces=np.asarray([p.vertices[:] for p in ob.data.polygons],dtype='<u4')
    if not np.array_equal(before,after) or not np.array_equal(faces,after_faces) or len(ob.modifiers):
        raise ValueError('publication rendering changed the imported mesh')
    scene_receipt={
        'schema':'jarambit.publication_scene.v1','subject':SUBJECT,
        'stl_sha256':hashlib.sha256(STL.read_bytes()).hexdigest(),
        'import_transform':{'uniform_scale':.001,'translation':[0,0,0],'rotation':[0,0,0]},
        'vertices':len(before),'triangles':len(faces),
        'imported_vertices_before_sha256':hashlib.sha256(before.tobytes()).hexdigest(),
        'imported_vertices_after_sha256':hashlib.sha256(after.tobytes()).hexdigest(),
        'faces_before_sha256':hashlib.sha256(faces.tobytes()).hexdigest(),
        'faces_after_sha256':hashlib.sha256(after_faces.tobytes()).hexdigest(),
        'mesh_modifiers':[],'geometry_changed':False,'shading':'original flat face normals',
        'blender':bpy.app.version_string,'build_hash':bpy.app.build_hash.decode(),
        'engine':'Cycles CPU','threads':4,'samples':args.samples,'seed':42,'denoise':True,
        'resolution':[args.width,args.height],'encoding':'lossless RGB PNG, 8 bit',
        'view_transform':s.view_settings.view_transform,'look':s.view_settings.look,
        'display_device':s.display_settings.display_device,'exposure':0,'gamma':1,
        'recipe':args.recipe,'style':style,'camera':{'type':'ORTHO','location':list(cam.location),
        'rotation_euler':list(cam.rotation_euler),'ortho_scale':cam.data.ortho_scale,'depth_of_field':False},
        'lights':[{'name':o.name,'location':list(o.location),'rotation_euler':list(o.rotation_euler),
                   'energy':o.data.energy,'size':o.data.size,'color':list(o.data.color)}
                  for o in s.objects if o.type=='LIGHT'],
        'field':'none; illustrative material finish, not FEM or manufactured surface evidence',
    }
    ocio=Path(bpy.utils.resource_path('LOCAL'))/'datafiles/colormanagement/config.ocio'
    if ocio.is_file():scene_receipt['ocio_config_sha256']=hashlib.sha256(ocio.read_bytes()).hexdigest()
    s.render.filepath=args.output;bpy.ops.render.render(write_still=True)
    after=np.asarray([v.co[:] for v in ob.data.vertices],dtype='<f4')
    after_faces=np.asarray([p.vertices[:] for p in ob.data.polygons],dtype='<u4')
    if not np.array_equal(before,after) or not np.array_equal(faces,after_faces):
        raise ValueError('rendering changed the imported mesh')
    scene_receipt['imported_vertices_after_sha256']=hashlib.sha256(after.tobytes()).hexdigest()
    scene_receipt['faces_after_sha256']=hashlib.sha256(after_faces.tobytes()).hexdigest()
    Path(args.output).with_suffix('.scene.json').write_text(json.dumps(scene_receipt,indent=2)+'\n')


if '--profile' in sys.argv and sys.argv[sys.argv.index('--profile')+1]=='publication':
    parser=argparse.ArgumentParser()
    parser.add_argument('--profile',choices=['publication'],required=True)
    parser.add_argument('--scene',choices=['hero_oblique'],required=True)
    parser.add_argument('--output',required=True)
    parser.add_argument('--recipe',choices=['neutral','dark','editorial'],required=True)
    parser.add_argument('--samples',type=int,required=True)
    parser.add_argument('--width',type=int,required=True)
    parser.add_argument('--height',type=int,required=True)
    publication_beauty(parser.parse_args(sys.argv[sys.argv.index('--')+1:]))
    sys.exit(0)

context = json.loads(Path(os.environ['JARAMBIT_RENDER_CONTEXT']).read_text())
atlas.SUBJECT = context['label']
atlas.STL = Path(context['artifacts']['stl'])
atlas.D1 = Path(context['artifacts']['d1'])
atlas.SCORE = json.loads(Path(context['artifacts']['scorecard']).read_text())
for name in ('hero_oblique', 'hero_reverse', 'hero_section'):
    title, subtitle, mode = atlas.SCENES[name]
    atlas.SCENES[name] = (title, 'Exact assessed export · ' + context['status'], mode)
atlas.main()
output = Path(sys.argv[sys.argv.index('--output')+1]).with_suffix('.labels.json')
captions = json.loads(output.read_text())
# The final two entries are the subject/verdict and qualification annotations in
# the retained scene library; the geometry pass contains no rasterized labels.
assert captions[-2]['text'].startswith(atlas.SUBJECT + '   /   ')
assert captions[-1]['text'] == 'ASSESSED EXAMPLE · NOT QUALIFIED · CHAMPION = NULL'
if 'RS-karambit-v2: FAIL' in captions[-2]['text']:
    captions[-2]['text'] = captions[-2]['text'].replace('RS-karambit-v2: FAIL',
        'RS-karambit-v2: ' + ('PASS' if context['rs_v2_all_hard'] else 'FAIL'))
captions[-1]['text'] = context['status']
output.write_text(json.dumps(captions, indent=1)+'\n')
