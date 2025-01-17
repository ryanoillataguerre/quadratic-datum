use super::*;

#[wasm_bindgen]
impl GridController {
    /// Returns cell data in a format useful for rendering. This includes only
    /// the data necessary to render raw text values.
    ///
    /// Returns a string containing a JSON array of [`JsRenderCell`].
    #[wasm_bindgen(js_name = "getRenderCells")]
    pub fn get_render_cells(&self, sheet_id: String, &rect: &Rect) -> Result<String, JsValue> {
        let Some(sheet) = self.try_sheet_from_string_id(sheet_id) else {
            return Result::Err("Sheet not found".into());
        };
        let output = sheet.get_render_cells(rect);
        Ok(serde_json::to_string::<[JsRenderCell]>(&output).map_err(|e| e.to_string())?)
    }

    /// Returns whether there is any cells to render in this region
    #[wasm_bindgen(js_name = "hasRenderCells")]
    pub fn has_render_cells(&self, sheet_id: String, &region: &Rect) -> bool {
        let Some(sheet) = self.try_sheet_from_string_id(sheet_id) else {
            return false;
        };
        sheet.has_render_cells(region)
    }
}
