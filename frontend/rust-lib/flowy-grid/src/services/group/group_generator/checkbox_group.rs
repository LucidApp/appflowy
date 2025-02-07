use crate::entities::{CheckboxGroupConfigurationPB, GroupRowsChangesetPB};

use flowy_grid_data_model::revision::{FieldRevision, RowChangeset, RowRevision};

use crate::services::field::{CheckboxCellData, CheckboxCellDataParser, CheckboxTypeOptionPB, CHECK, UNCHECK};
use crate::services::group::{GenericGroupController, Group, GroupController, GroupGenerator, Groupable};

pub type CheckboxGroupController = GenericGroupController<
    CheckboxGroupConfigurationPB,
    CheckboxTypeOptionPB,
    CheckboxGroupGenerator,
    CheckboxCellDataParser,
>;

impl Groupable for CheckboxGroupController {
    type CellDataType = CheckboxCellData;

    fn can_group(&self, _content: &str, _cell_data: &Self::CellDataType) -> bool {
        false
    }

    fn add_row_if_match(
        &mut self,
        _row_rev: &RowRevision,
        _cell_data: &Self::CellDataType,
    ) -> Vec<GroupRowsChangesetPB> {
        todo!()
    }

    fn remove_row_if_match(
        &mut self,
        _row_rev: &RowRevision,
        _cell_data: &Self::CellDataType,
    ) -> Vec<GroupRowsChangesetPB> {
        todo!()
    }

    fn move_row_if_match(
        &mut self,
        _field_rev: &FieldRevision,
        _row_rev: &RowRevision,
        _row_changeset: &mut RowChangeset,
        _cell_data: &Self::CellDataType,
        _to_row_id: &str,
    ) -> Vec<GroupRowsChangesetPB> {
        todo!()
    }
}

impl GroupController for CheckboxGroupController {
    fn will_create_row(&mut self, _row_rev: &mut RowRevision, _field_rev: &FieldRevision, _group_id: &str) {
        todo!()
    }
}

pub struct CheckboxGroupGenerator();
impl GroupGenerator for CheckboxGroupGenerator {
    type ConfigurationType = CheckboxGroupConfigurationPB;
    type TypeOptionType = CheckboxTypeOptionPB;

    fn generate_groups(
        field_id: &str,
        _configuration: &Option<Self::ConfigurationType>,
        _type_option: &Option<Self::TypeOptionType>,
    ) -> Vec<Group> {
        let check_group = Group::new(
            "true".to_string(),
            field_id.to_owned(),
            "".to_string(),
            CHECK.to_string(),
        );
        let uncheck_group = Group::new(
            "false".to_string(),
            field_id.to_owned(),
            "".to_string(),
            UNCHECK.to_string(),
        );
        vec![check_group, uncheck_group]
    }
}
