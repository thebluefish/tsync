use crate::BuildState;

impl super::ToTypescript for syn::ItemType {
    fn convert_to_ts(self, state: &mut BuildState, _debug: bool, uses_typeinterface: bool) {
        let export = if uses_typeinterface { "" } else { "export " };
        state.types.push('\n');
        let name = self.ident.to_string();
        let ty = crate::typescript::convert_type(&self.ty);
        let comments = crate::utils::get_comments(self.attrs);
        state.write_comments(&comments, 0);
        state
            .types
            .push_str(format!("{export}type {name} = {ty}", name = name, ty = ty.ts_type).as_str());

        state.types.push('\n');
    }
}
