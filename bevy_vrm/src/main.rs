use bevy::{asset::AssetLoader, prelude::*};

use bevy_vrm::{BoneName, HumanoidBones, VrmBundle, VrmPlugin};

use std::f32::consts::PI;

use vmc_listener::{
    VMCListener, BLEND_APPLY_ADDR, BLEND_TRACKING_ADDR, BONE_TRACKING_ADDR, CAMERA_ADDR,
    EYE_TRACKING_ADDR, ROOT_ADDR,
};

#[derive(Resource)]
pub struct PendingBlendShapes {
    shapes: Vec<(String, f32)>,
}

fn get_bone_name<S: Into<String>>(bone_text: S) -> Option<BoneName> {
    let name = bone_text.into();

    Some(match name.as_str() {
        "RightFoot" => BoneName::RightFoot,
        "LeftFoot" => BoneName::LeftFoot,
        "RightHand" => BoneName::RightHand,
        "LeftHand" => BoneName::LeftHand,
        //"Pelvis" => BoneName::Hips, // TODO: WUT
        "Hips" => BoneName::Hips,
        "Spine" => BoneName::Spine,
        "Head" => BoneName::Head,
        "UpperChest" => BoneName::UpperChest,
        "RightLittleDistal" => BoneName::RightLittleDistal,
        "RightLittleIntermediate" => BoneName::RightLittleIntermediate,
        "RightLittleProximal" => BoneName::RightLittleProximal,
        "RightRingDistal" => BoneName::RightRingDistal,
        "RightRingIntermediate" => BoneName::RightRingIntermediate,
        "RightRightPromixel" => BoneName::RightRingProximal,
        "RightMiddleDistal" => BoneName::RightMiddleDistal,
        "RightMiddleIntermediate" => BoneName::RightMiddleIntermediate,
        "RightMiddleProximal" => BoneName::RightMiddleProximal,
        "RightIndexDistal" => BoneName::RightIndexDistal,
        "RightIndexIntermediate" => BoneName::RightIndexIntermediate,
        "RightIndexProximal" => BoneName::RightIndexProximal,
        "RightThumbDistal" => BoneName::RightThumbDistal,
        "RightThumbIntermediate" => BoneName::RightThumbIntermediate,
        "RightThumbProximal" => BoneName::RightThumbProximal,
        "LeftLittleDistal" => BoneName::LeftLittleDistal,
        "LeftLittleIntermediate" => BoneName::LeftLittleIntermediate,
        "LeftLittleProximal" => BoneName::LeftLittleProximal,
        "LeftRingDistal" => BoneName::LeftRingDistal,
        "LeftRingIntermediate" => BoneName::LeftRingIntermediate,
        "LeftRightPromixel" => BoneName::LeftRingProximal,
        "LeftMiddleDistal" => BoneName::LeftMiddleDistal,
        "LeftMiddleIntermediate" => BoneName::LeftMiddleIntermediate,
        "LeftMiddleProximal" => BoneName::LeftMiddleProximal,
        "LeftIndexDistal" => BoneName::LeftIndexDistal,
        "LeftIndexIntermediate" => BoneName::LeftIndexIntermediate,
        "LeftIndexProximal" => BoneName::LeftIndexProximal,
        "LeftThumbDistal" => BoneName::LeftThumbDistal,
        "LeftThumbIntermediate" => BoneName::LeftThumbIntermediate,
        "LeftThumbProximal" => BoneName::LeftThumbProximal,
        "RightEye" => BoneName::RightEye,
        "LeftEye" => BoneName::LeftEye,
        "RightToes" => BoneName::RightToes,
        "LeftToes" => BoneName::LeftToes,
        "RightLowerArm" => BoneName::RightLowerArm,
        "LeftLowerArm" => BoneName::LeftLowerArm,
        "RightUpperArm" => BoneName::RightUpperArm,
        "LeftUpperArm" => BoneName::LeftUpperArm,
        "RightShoulder" => BoneName::RightShoulder,
        "LeftShoulder" => BoneName::LeftShoulder,
        "Neck" => BoneName::Neck,
        "Chest" => BoneName::Chest,
        "RightLowerLeg" => BoneName::RightLowerLeg,
        "LeftLowerLeg" => BoneName::LeftLowerLeg,
        "RightUpperLeg" => BoneName::RightUpperLeg,
        "LeftUpperLeg" => BoneName::LeftUpperLeg,
        _ => return None,
    })
}

fn main() {
    println!("Hello, world!");

    let mut listener = VMCListener::new();
    listener.ready();

    let mut app = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VrmPlugin)
        .insert_resource(listener)
        .insert_resource(PendingBlendShapes { shapes: Vec::new() })
        .add_systems(Startup, setup)
        .add_systems(Update, update_model)
        .run();
}

fn update_model(
    mut listener: ResMut<VMCListener>,
    mut humanoid_bones: Query<&mut HumanoidBones>,
    mut transform: Query<&mut Transform>,
    mut pending_blend_shapes: ResMut<PendingBlendShapes>,
) {
    for message in &mut listener.process() {
        //  println!("{:#?}", message.address);
        if message.address.contains(ROOT_ADDR) {
            let name = message.get_string(0).unwrap();
            let quat = Quat::from_xyzw(
                message.get_float(4).unwrap(),
                -message.get_float(5).unwrap(),
                -message.get_float(6).unwrap(),
                message.get_float(7).unwrap(),
            );

            let position = Vec3::new(
                message.get_float(1).unwrap(),
                message.get_float(2).unwrap(),
                message.get_float(3).unwrap(),
            );

            for bone in &mut humanoid_bones {
                if let Some(bone_name) = get_bone_name(&name) {
                    if let Some(bone_entity) = bone.0.get(&bone_name) {
                        if let Ok(mut bone_transform) = transform.get_mut(*bone_entity) {
                            println!("Root bone was used!");
                            bone_transform.translation = position;
                            bone_transform.rotation = quat;
                        }
                    }
                }
            }
        }

        if message.address.contains(BONE_TRACKING_ADDR) {
            let name = message.get_string(0).unwrap();
            let x = -message.get_float(4).unwrap();
            let y = message.get_float(5).unwrap();
            let z = message.get_float(6).unwrap();
            let w = message.get_float(7).unwrap();

            let position = Vec3::new(
                message.get_float(1).unwrap(),
                message.get_float(2).unwrap(),
                message.get_float(3).unwrap(),
            );

            let mut quat = Quat::from_xyzw(x, y, z, w);

            if quat.length_squared() > f32::EPSILON {
                quat = quat.normalize();
            }
            if !quat.is_normalized() {
                quat = Quat::IDENTITY; // new(0.0, 0.0, 0.0, 1.0);
            }
            for bone in &mut humanoid_bones {
                if let Some(bone_name) = get_bone_name(&name) {
                    if let Some(bone_entity) = bone.0.get(&bone_name) {
                        if let Ok(mut bone_transform) = transform.get_mut(*bone_entity) {
                            bone_transform.translation = position;
                            bone_transform.rotation = quat;
                        }
                    }
                }
            }
        }

        if message.address.contains(BLEND_TRACKING_ADDR) {
            //          //println!("Blend tracking");
            let mut name = message.get_string(0).unwrap();
            if listener.blend_shape_translations.contains_key(&name) {
                //println!("Has translation");
                //println!("Before: {}", name);
                name = listener.blend_shape_translations.get(&name).unwrap().into();
                //println!("After: {}", name);
            }

            //          //println!("BLEND APPLY: {} {}", name, message.get_float(1).unwrap());
            pending_blend_shapes
                .shapes
                .push((name, message.get_float(1).unwrap()));
        }
        if message.address.contains(BLEND_APPLY_ADDR) {
            for (name, value) in &pending_blend_shapes.shapes {}

            // for (name, float) in &blend_shapes_to_apply {
            //          for (mesh, animation) in &self.mesh_blend_shapes {
            //            //println!("ANIMATIONS: {:?}", animation);
            //            if let Some(anim) = animation.get(name) {
            //              for index in anim {
            //                //println!("MESH: {}", mesh);
            //                let mut mesh: Gd<MeshInstance3D> = self.base().get_node_as(mesh);
            //                //println!("Index: {} float: {}", name, *float);
            //                mesh.set_blend_shape_value(*index, *float);
            //                //self
            //                //  .base()
            //                //  .get(mesh.into())
            //                //  .to::<Gd<MeshInstance3D>>()
            //                //  //.get_node_as::<Gd<MeshInstance3D>>()
            //                //  .set_blend_shape_value(*index, *float);
            //              }
            //            } else {
            //              //println!("NO BLENDSHAPE ANIMATION FOR: {}: {}", *mesh, name);
            //            }
            //          }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 2.0, 5.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-PI / 3.0)),
        ..default()
    });

    commands.spawn(VrmBundle {
        scene_bundle: SceneBundle {
            transform: Transform::from_xyz(1.0, 0.8, 4.0).with_rotation(Quat::from_rotation_y(PI)),
            ..default()
        },
        vrm: asset_server.load("owlkaline.vrm"),
        ..default()
    });
}
