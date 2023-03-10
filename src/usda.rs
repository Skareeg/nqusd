// General concept structure.

// Stage is the outermost inmemory container.
// Can open or create a new stage.
// Python API: stage = Usd.Stage.Open(path) / Usd.Stage.CreateNew(path)
// Can save a stage.
// Python API: stage.Save()
// Can export to a different file and/or format.
// Python API: stage.Export(path)
// Temporary stage in memory.
// Python API: stage = Usd.Stage.CreateInMemory(name)
// Flatten all of the layers and references. (Returns a final baked layer, no variants or refs.)
// Python API: stage.Flatten()

// Layer is the file itself.
// It contains PrimSpecs, that have Property and Metadata values.
// It has an identifier that can be used to make references from other layers.
// They must currently correspond to files on a POSIX interface filesystem.

// Example APIs
// xformPrim = UsdGeom.Xform.Define(stage, "/TestSphere")
// UsdGeom.XformCommonAPI(xformPrim).SetTranslate((1, 2, 3))
// spherePrim = UsdGeom.Sphere.Define(stage, "/TestSphere/MeshData")
// sphere = stage.GetPrimAtPath("/TestSphere/MeshData") # Retrieves the sphere prim as a generic one.
// radiusAttr = sphere.GetAttribute("radius")
// extentAttr = sphere.GetAttribute("extent")
// colorAttr = spherePrim.GetDisplayColorAttr() # Need the UsdGeom.Sphere API for the color attribute.
// print(stage.GetRootLayer().ExportToString()) # Pretty prints the stage and all it's layers out.
// stage.Export("./TestLayer.usda", addSourceFileComment=False)

// General parsing structure.

// Starter marker of USDA file.
// #usda 1.0

// def [type] <name>
// {
//    ...
// }
// [type] can be infered if it is a reference (see below).
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

// Higher file sublayering of lower files:
// (
//    subLayers = [
//       @./overriding_scene.usda@,
//       @./original_scene.usda@
//    ]
// )

// Referencing:
// def <name> (references = <location>) {}
// e.g. def "CubeRef1" ( references = @./original_scene.usda@</World/Cube> ) { ... fieldnames to override or add ... }
// References might be an array (e.g. references = [...]). Need to investigate.

// Variants:
// def <type> <name> ( variants = { string <variantname> = <variantdefaultvalue> } prepend variantSets = <variantsetname>) { def [type] <baseobjectname> [(refinfo)] {} variantSet <variantsetname> = { <variantname> { over <baseobjectname> { ... overriding or added fields ... } } .. * } }

// Selecting variants on reference:
// def <name> ( references = <location> variants = { string <variantname> = <variantvalue> } ) { ... overriding or added fields ... }