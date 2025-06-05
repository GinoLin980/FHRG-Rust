use std::collections::HashMap;

// ['"s32",', '"u32",', '"f32",', '"hzn",', '"u16",', '"u8",', '"s8",']
pub enum DataType {
    Int(i32),
    UInt(u32),
    Float(f32),
    UShort(u16),
    UByte(u8),
    SByte(i8),
}

impl DataType {
    pub fn data_size(&self) -> usize {
        match self {
            DataType::Int(_) => std::mem::size_of::<i32>(),   // 4
            DataType::UInt(_) => std::mem::size_of::<u32>(),  // 4
            DataType::Float(_) => std::mem::size_of::<f32>(), // 4
            DataType::UShort(_) => std::mem::size_of::<u16>(), // 2
            DataType::UByte(_) => std::mem::size_of::<u8>(),  // 1
            DataType::SByte(_) => std::mem::size_of::<i8>(),  // 1
        }
    }

    pub fn size_from_str(name: &str) -> Option<usize> {
        match name {
            "s32" => Some(4),
            "u32" => Some(4),
            "f32" => Some(4),
            "hzn" => Some(12),
            "u16" => Some(2),
            "u8" => Some(1),
            "s8" => Some(1),
            _ => None,
        }
    }
}

pub fn decode_data(data: &[u8]) -> HashMap<String, DataType> {
    let mut return_map: HashMap<String, DataType> = HashMap::new();
    let data_types = init_global();
    let mut offset = 0;

    for (key, value) in &data_types {
        let jump = match DataType::size_from_str(value) {
            Some(size) => size,
            None => continue,
        };

        if offset + jump > data.len() {
            eprintln!("Not enough data for field {}", key);
            continue;
        }

        let current_data = &data[offset..offset + jump];
        offset += jump;

        let decoded = match *value {
            "s32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::Int(i32::from_le_bytes(bytes))
            },
            "u32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::UInt(u32::from_le_bytes(bytes))
            },
            "f32" => {
                let bytes: [u8; 4] = current_data.try_into().unwrap();
                DataType::Float(f32:: from_le_bytes(bytes))
            },
            "u16" => {
                let bytes: [u8; 2] = current_data.try_into().unwrap();
                DataType::UShort(u16:: from_le_bytes(bytes))
            },
            "u8" => {
                let bytes: [u8; 1] = current_data.try_into().unwrap();
                DataType::UByte(bytes[0])
            },
            "s8" => {
                let bytes: [u8; 1] = current_data.try_into().unwrap();
                DataType::SByte(i8::from_le_bytes(bytes))
            },
            _ => {
                // eprintln!("Unknown data => {}: {}", key, value);
                DataType::Int(0)
            }
        };
        return_map.insert(key.to_string(), decoded);
    }
    return_map
}

fn init_global() -> Vec<(&'static str, &'static str)> {
    vec![
        ("IsRaceOn", "s32"),
        ("TimestampMS", "u32"),
        ("EngineMaxRpm", "f32"),
        ("EngineIdleRpm", "f32"),
        ("CurrentEngineRpm", "f32"),
        ("AccelerationX", "f32"),
        ("AccelerationY", "f32"),
        ("AccelerationZ", "f32"),
        ("VelocityX", "f32"),
        ("VelocityY", "f32"),
        ("VelocityZ", "f32"),
        ("AngularVelocityX", "f32"),
        ("AngularVelocityY", "f32"),
        ("AngularVelocityZ", "f32"),
        ("Yaw", "f32"),
        ("Pitch", "f32"),
        ("Roll", "f32"),
        ("NormalizedSuspensionTravelFrontLeft", "f32"),
        ("NormalizedSuspensionTravelFrontRight", "f32"),
        ("NormalizedSuspensionTravelRearLeft", "f32"),
        ("NormalizedSuspensionTravelRearRight", "f32"),
        ("TireSlipRatioFrontLeft", "f32"),
        ("TireSlipRatioFrontRight", "f32"),
        ("TireSlipRatioRearLeft", "f32"),
        ("TireSlipRatioRearRight", "f32"),
        ("WheelRotationSpeedFrontLeft", "f32"),
        ("WheelRotationSpeedFrontRight", "f32"),
        ("WheelRotationSpeedRearLeft", "f32"),
        ("WheelRotationSpeedRearRight", "f32"),
        ("WheelOnRumbleStripFrontLeft", "s32"),
        ("WheelOnRumbleStripFrontRight", "s32"),
        ("WheelOnRumbleStripRearLeft", "s32"),
        ("WheelOnRumbleStripRearRight", "s32"),
        ("WheelInPuddleDepthFrontLeft", "f32"),
        ("WheelInPuddleDepthFrontRight", "f32"),
        ("WheelInPuddleDepthRearLeft", "f32"),
        ("WheelInPuddleDepthRearRight", "f32"),
        ("SurfaceRumbleFrontLeft", "f32"),
        ("SurfaceRumbleFrontRight", "f32"),
        ("SurfaceRumbleRearLeft", "f32"),
        ("SurfaceRumbleRearRight", "f32"),
        ("TireSlipAngleFrontLeft", "f32"),
        ("TireSlipAngleFrontRight", "f32"),
        ("TireSlipAngleRearLeft", "f32"),
        ("TireSlipAngleRearRight", "f32"),
        ("TireCombinedSlipFrontLeft", "f32"),
        ("TireCombinedSlipFrontRight", "f32"),
        ("TireCombinedSlipRearLeft", "f32"),
        ("TireCombinedSlipRearRight", "f32"),
        ("SuspensionTravelMetersFrontLeft", "f32"),
        ("SuspensionTravelMetersFrontRight", "f32"),
        ("SuspensionTravelMetersRearLeft", "f32"),
        ("SuspensionTravelMetersRearRight", "f32"),
        ("CarOrdinal", "s32"),
        ("CarClass", "s32"),
        ("CarPerformanceIndex", "s32"),
        ("DrivetrainType", "s32"),
        ("NumCylinders", "s32"),
        ("HorizonPlaceholder", "hzn"),
        ("PositionX", "f32"),
        ("PositionY", "f32"),
        ("PositionZ", "f32"),
        ("Speed", "f32"),
        ("Power", "f32"),
        ("Torque", "f32"),
        ("TireTempFrontLeft", "f32"),
        ("TireTempFrontRight", "f32"),
        ("TireTempRearLeft", "f32"),
        ("TireTempRearRight", "f32"),
        ("Boost", "f32"),
        ("Fuel", "f32"),
        ("DistanceTraveled", "f32"),
        ("BestLap", "f32"),
        ("LastLap", "f32"),
        ("CurrentLap", "f32"),
        ("CurrentRaceTime", "f32"),
        ("LapNumber", "u16"),
        ("RacePosition", "u8"),
        ("Accel", "u8"),
        ("Brake", "u8"),
        ("Clutch", "u8"),
        ("HandBrake", "u8"),
        ("Gear", "u8"),
        ("Steer", "s8"),
        ("NormalizedDrivingLine", "s8"),
        ("NormalizedAIBrakeDifference", "s8"),
    ]
}
