/// 定义类 C 的枚举类型，并通过一些函数将外部不可控数据安全地转换为枚举类型。
/// 
/// ⚠️ 只是适用于含单个 primitive 类型字段的结构体。
///
/// 示例：
///
/// ```
/// use const_enum::const_enum;
///
/// pub struct Hello {
///     pub data: u8,
/// }
///
/// const_enum! {
///     pub HelloEnum [Hello::data:u8] {
///         V0: 0,
///         V1: 1,
///         V2: 12,
///         V3: 13,
///         V4: 22
///     }
/// }
///
/// ```
/// 上述代码会生成：
/// ```
/// pub struct Hello {
///     data: u8,
/// }
/// #[repr(u8)]
/// pub enum HelloEnum {
///     V0 = 0,
///     V1 = 1,
///     V2 = 12,
///     V3 = 13,
///     V4 = 22,
/// }
/// impl core::convert::Into<u8> for HelloEnum {
///     fn into(self) -> u8 {
///         self as u8
///     }
/// }
/// impl Hello {
///     pub fn as_enum(&self) -> Result<HelloEnum, u8> {
///         match self.data {
///             0 => Ok(HelloEnum::V0),
///             1 => Ok(HelloEnum::V1),
///             12 => Ok(HelloEnum::V2),
///             13 => Ok(HelloEnum::V3),
///             22 => Ok(HelloEnum::V4),
///             _ => Err(self.data),
///         }
///     }
/// }
/// impl Hello {
///     pub const V0: Hello = Hello { data: 0 };
///     pub const V1: Hello = Hello { data: 1 };
///     pub const V2: Hello = Hello { data: 12 };
///     pub const V3: Hello = Hello { data: 13 };
///     pub const V4: Hello = Hello { data: 22 };
/// }
/// ```
///
/// ❗ 不允许使用 range，如：
///
/// ```compile_fail
/// use const_enum::const_enum;
///
/// pub struct Hello {
///     pub data: u8,
/// }
///
/// const_enum! {
///     pub HelloEnum [Hello::data:u8] {
///         V0: 0,
///         V1: 1,
///         V2: 12,
///         V3: 13 ..= 19,
///         V4: 22
///     }
/// }
///
/// ```
///
/// 理由是：
/// enum 类型会将非法 range 暴露，但是没有任何有效手段阻止。例如：
///
/// ```
/// enum Example {
///     V0(u8),
/// }
/// ```
///
/// 我们希望 `Example::V0` 在 `0..=4` 范围内，实际上由于 V0 会随着 Example 的暴露而暴露，用户可以直接赋值，而我们却没办法检查其合法性。
///
/// 另外需要确保 `Hello::data` 是私有的，并且不提供修改的外部接口。这样能够保证数据安全。
///
/// 关于如何简便使用 `as_enum` 的一个示例：
///
/// ```
/// use const_enum::const_enum;
///
/// # pub struct Hello {
/// #     pub data: u8,
/// # }
/// # const_enum! {
/// #     pub HelloEnum [Hello::data:u8] {
/// #         V0: 0,
/// #         V1: 1,
/// #         V2: 12,
/// #         V3: 13,
/// #         V4: 22
/// #     }
/// # }
/// pub fn example(e: Hello) -> ! {
///     if let Ok(v) = e.as_enum() {
///         match v {
///             HelloEnum::V0 => todo!(),
///             HelloEnum::V1 => todo!(),
///             HelloEnum::V2 => todo!(),
///             HelloEnum::V3 => todo!(),
///             HelloEnum::V4 => todo!(),
///         }
///     } else {
///         todo!()
///     }
/// }
/// ```
#[macro_export]
macro_rules! const_enum {
    (
        $($Vis:vis $EnumType:ident [$Struct:ident::$Field:ident: $FieldType:tt] {
            $(
                $(#[$Doc:meta])?
                $Variance:ident : $Value:literal
            ),+
        })+
    ) => {
        $(
            $crate::const_enum!{
                def_enum: $Vis $EnumType, $FieldType,
                $($(#[$Doc])? $Variance $Value),+
            }
            $crate::const_enum!{
                def_const:
                $Vis $Struct::$Field, $FieldType,
                $($(#[$Doc])? $Variance $Value),+
            }
            $crate::const_enum!{
                into_struct:
                $EnumType, $Struct::$Field:$FieldType,
                $($Variance $Value),+
            }
            $crate::const_enum!{
                as_enum:
                $Vis $Struct::$Field, $EnumType, $FieldType,
                $($Variance $Value),+
            }
        )+
    };
    (def_enum: $Vis:vis $EnumType:ident, $FieldType:tt, $($(#[$Doc:meta])? $Variance:ident $Value:literal),+) => {
        #[repr($FieldType)]
        $Vis enum $EnumType {
            $(
                $(#[$Doc])?
                $Variance = $Value
            ),+
        }
    };
    (into_struct: $EnumType:ident, $Struct:ident::$Field:ident:$FieldType:ty, $($Variance:ident $Value:literal),+) => {
        impl core::convert::Into<$Struct> for $EnumType {
            fn into(self) -> $Struct {
                    $Struct {
                        $Field: self as $FieldType
                    }
            }
        }
    };
    (as_enum: $Vis:vis $Struct:ident::$Field:ident, $EnumType:ident, $FieldType:ty, $($Variance:ident $Value:literal),+) => {
        impl $Struct {
            $Vis fn as_enum(&self) -> Result<$EnumType, $FieldType> {
                match self.$Field {
                    $(
                        $Value => Ok($EnumType::$Variance),
                    )+
                    _ => Err(self.$Field)
                }
            }
        }
    };
    (def_const: $Vis:vis $Struct:ident::$Field:ident, $FieldType:ty, $($(#[$Doc:meta])? $Variance:ident $Value:literal),+) => {
        impl $Struct {
            $(
                $(#[$Doc])?
                $Vis const $Variance: $Struct = $Struct { $Field:$Value };
            )+
        }
    };
}
