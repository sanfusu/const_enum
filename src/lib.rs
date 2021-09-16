/// 定义类 C 的枚举类型，并通过一些函数将外部不可控数据安全地转换为枚举类型。
///
/// ⚠️ 只是适用于含单个 primitive 类型字段的结构体。
///
/// 示例：
///
/// ```
/// use const_enum::{
///     const_enum, AsEnum,
///     ConstEnum::{self, Wellknown, Unknown}
/// };
///
/// pub struct Hello {
///     pub data: u8,
/// }
///
/// const_enum! {
///     pub Hellos [Hello::data:u8, 0..=22] {  // 0..=22(可选) 为常量范围，具有更改优先级。
///         V0: 0,
///         V1: 1,
///         V2: 12,
///         V3: 13,
///         V4: 22
///     }
/// }
///
/// let hello = Hello {data: 33};
///
/// match hello.as_enum() {
///     Wellknown(v) => {
///         match v {
///             Hellos::V0 => {},
///             Hellos::V1 => {},
///             _ => {},
///         }
///     }
///     Unknown(v) => {
///         match v {
///             33 => {},
///             _ => {}
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! const_enum {
    (
        $($Vis:vis $EnumType:ident [$Struct:ident$(($SuperStruct:ident))?::$Field:ident: $FieldType:tt $(,$Low:literal ..= $Upper:literal)?] {
            $(
                $(#[$Doc:meta])?
                $Variance:ident : $Value:literal
            ),+ $(,)?
        })+
    ) => {
        $(
            $crate::const_enum!{
                def_enum: $Vis $EnumType, $FieldType,
                $($(#[$Doc])? $Variance $Value),+
            }
            $crate::const_enum!{
                into_struct:
                $EnumType, $Struct$(($SuperStruct))?::$Field:$FieldType,
                $($Variance $Value),+
            }
            $crate::const_enum!{
                as_enum:
                $Vis $Struct::$Field, $EnumType, $FieldType, $($Low ..= $Upper,)?
                $($Variance $Value),+
            }
        )+
    };
    (def_enum: $Vis:vis $EnumType:ident, $FieldType:tt, $($(#[$Doc:meta])? $Variance:ident $Value:literal),+) => {
        #[repr($FieldType)]
        #[derive(Debug)]
        $Vis enum $EnumType {
            $(
                $(#[$Doc])?
                $Variance = $Value
            ),+
        }
    };
    (into_struct: $EnumType:ident,$Struct:ident $(($SuperStruct:ident))?::$Field:ident:$FieldType:ty, $($Variance:ident $Value:literal),+) => {
        impl core::convert::Into<$Struct> for $EnumType {
            #[inline]
            fn into(self) -> $Struct {
                    $Struct {
                        $Field: self as $FieldType
                    }
            }
        }
        $(
            impl core::convert::Into<$SuperStruct> for $EnumType {
                #[inline]
                fn into(self) -> $SuperStruct {
                    $SuperStruct {
                        $Field: self as $FieldType
                    }
                }
            }
        )?
    };
    (as_enum: $Vis:vis $Struct:ident::$Field:ident, $EnumType:ident, $FieldType:ty, $($Low:literal ..= $Upper:literal,)? $($Variance:ident $Value:literal),+ ) => {
        impl $crate::AsEnum for $Struct {
            type TargetEnum = $EnumType;
            type BaseType = $FieldType;
            #[inline]
            fn as_enum(&self) -> $crate::ConstEnum<$EnumType, $FieldType> {
                $(
                    if !($Low..=$Upper).contains(&self.$Field) {
                        return $crate::ConstEnum::Unknown(self.$Field);
                    }
                )?
                match self.$Field {
                    $(
                        $Value => $crate::ConstEnum::Wellknown($EnumType::$Variance),
                    )+
                    _ => $crate::ConstEnum::Unknown(self.$Field)
                }
            }
        }
    };
}

pub enum ConstEnum<TargetEnum, BaseType> {
    Wellknown(TargetEnum),
    Unknown(BaseType),
}

impl<TragetEnum, BaseType: core::fmt::Debug> ConstEnum<TragetEnum, BaseType> {
    pub fn unwrap(self) -> TragetEnum {
        match self {
            ConstEnum::Wellknown(v) => v,
            ConstEnum::Unknown(v) => panic!("Unknown value {:?}", v),
        }
    }
}

pub trait AsEnum {
    type TargetEnum;
    type BaseType: Copy;
    fn as_enum(&self) -> ConstEnum<Self::TargetEnum, Self::BaseType>;
}

pub struct Hello {
    pub data: u8,
}

const_enum! {
    pub HelloEnum [Hello::data:u8, 0..=22] {  // 0..=22 为常量范围，具有更改优先级。
        V0: 0,
        V1: 1,
        V2: 12,
        V3: 13,
        V4: 22
    }
}

pub use self::ConstEnum::Unknown;
pub use self::ConstEnum::Wellknown;
