use bevy::prelude::Bundle;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Bundle,)]
pub struct InputHandlingBundle<Keys>
where
    Keys: BindingTypeView,
{
    pub view: InputView<Keys>,
}
