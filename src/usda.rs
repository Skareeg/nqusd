
// General parsing structure.

// def [type] <name>
// {
//    ...
// }
// [type] can be infered if it is a reference.
// e.g. def Xform "World"
// e.g. def Mesh "Cube"

// {
//    <datatype>[[]] <fieldname> = <value/array>
// }
// e.g. { float3[] extent = [(-1.0, -1.0, -1.0), (1.0, 1.0, 1.0)] }
// e.g. { int[] faceVertexCounts = [3, 3] int[] faceVertexIndices = [..] }

// Known current types:
// Xform
// Mesh

// Known current datatypes:
// float3
// int
// point3f
// color3f
// double3
// uniform token

// Composition of sublayering:
// over <type> <name> {}
// This strengthens this unit ontop of a any conflicting files based on order in the upper layer file.
// e.g.:
//       full_scene.usda
//       - overriding_scene.usda
//       - original_scene.usda