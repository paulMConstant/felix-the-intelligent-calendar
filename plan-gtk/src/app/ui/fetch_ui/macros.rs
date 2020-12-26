/// Fetches the gtk component with given name from the builder.
macro_rules! fetch_ui_from_builder {
    ($from: expr, $id: literal) => {
        $from
            .builder
            .get_object($id)
            .expect(&format!("Could not get {} from ui file", $id))
    };
}
