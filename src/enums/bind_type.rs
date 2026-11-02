macro_rules! bind_types {
    (
        $(
            $variant:ident = $id:expr => $name:expr
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum BindType {
            $(
                $variant = $id,
            )*
        }

        impl BindType {
            pub fn from_id(id: &u16) -> Option<Self> {
                match id {
                    $(
                        $id => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn to_id(&self) -> u16 {
                match self {
                    $(
                        Self::$variant => $id,
                    )*
                }
            }

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $name,
                    )*
                }
            }
        }
    };
}

bind_types! {
    None = 255 => "None",
    AccountBoundOnPickup = 1 => "Bound on Pickup",
    AccountBoundOnEquip = 2 => "Bind on Equip",
    CharacterBoundOnPickup = 3 => "Character-bound",
}