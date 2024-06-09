from Triangle import Triangle
from Vector3D import Vector3D

def parse_obj_file(file_path):
    vertices = []
    triangles = []

    with open(file_path, 'r') as file:
        for line in file:
            parts = line.strip().split()

            if not parts:
                continue  # Skip empty lines

            if parts[0] == 'v':
                # Vertex information
                vertex = list(map(float, parts[1:]))
                vertices.append(vertex)

            elif parts[0] == 'f':
                # Face information
                face = [int(index.split('/')[0]) for index in parts[1:]]
                triangles.append(face)

    return vertices, triangles

# Example usage
obj_file_path = './rolling-cube/1.obj'
vertices, triangles = parse_obj_file(obj_file_path)

# Display the parsed data
# print("Vertices:", vertices[1])
# print("Triangles:", triangles[1])

triangle_objects = []
for (i1, i2, i3, i4) in triangles:
    try:
        # Ensure indices are within bounds
        i1, i2, i3 = i1 - 1, i2 - 1, i3 - 1

        # Check if indices are valid
        if 0 <= i1 < len(vertices) and 0 <= i2 < len(vertices) and 0 <= i3 < len(vertices):
            triangle_objects.append(Triangle(
                Vector3D(*vertices[i1]),
                Vector3D(*vertices[i2]),
                Vector3D(*vertices[i3]),
                'orange'
            ))
        else:
            print("Invalid indices:", i1, i2, i3)

    except Exception as e:
        print(e)

# print(triangle_objects[2])