import pygltflib
import numpy as np
def read_gltf(file_path):
    # Load the GLTF file
    gltf = pygltflib.GLTF2().load(file_path)
    current_scene = gltf.scenes[gltf.scene]
    node_index = current_scene.nodes[0]  # scene.nodes is the indices, not the objects 
    box = gltf.nodes[node_index]
    for b in gltf.nodes:
        print(b)

def create_meshes_from_glb(filename, mesh_index=0):
    glb = pygltflib.GLTF2().load(filename)
    meshes = []
    # Assuming there is only one mesh in the GLTF file
    mesh = glb.meshes[0]

    # Access vertices
    attributes = mesh.primitives[0].attributes
    positions_accessor = attributes['POSITION']
    vertices = positions_accessor.get_data()
    print(vertices, indices)
    for prim in glb.meshes[mesh_index].primitives:

        # Indices
        accessor = glb.accessors[prim.indices]
        assert accessor.type == "SCALAR"
        assert not accessor.sparse
        assert accessor.componentType == pygltflib.UNSIGNED_SHORT
        nindices = accessor.count
        bv = glb.bufferViews[accessor.bufferView]
        data = glb._glb_data[bv.byteOffset : bv.byteOffset + bv.byteLength]
        triangles = np.frombuffer(data, dtype=np.uint16)
        triangles = np.reshape(triangles, (-1, 3))
        assert nindices == len(triangles) * 3

        # Vertices
        accessor = glb.accessors[prim.attributes.POSITION]
        assert accessor.type == "VEC3"
        assert not accessor.sparse
        assert accessor.componentType == pygltflib.FLOAT
        nvertices = accessor.count
        bv = glb.bufferViews[accessor.bufferView]
        data = glb._glb_data[bv.byteOffset : bv.byteOffset + bv.byteLength]
        vertices = np.frombuffer(data, dtype=np.float32)
        vertices = np.reshape(vertices, (-1, 3))
        assert nvertices == len(vertices)

        # print(prim)
        print(vertices[0])
        print(triangles[0])

        # faces = scale * vertices[triangles]
        # cull = lambda face_index, winding: None if winding < 0 else {}
        # meshes.append(svg3d.Mesh(faces, cull, style=style))

    # return meshes

# Example usage
gltf_file_path = "/Users/mmuhammad/Downloads/uploads_files_4253875_low+poly+guy.glb"
# read_gltf(gltf_file_path)
create_meshes_from_glb(gltf_file_path)
