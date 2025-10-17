# Design notes — fill_prompt

- API exposes `fill_template(&str, impl IntoIterator<Item=(K,V)>) -> Result<String, FillError>`.
- Placeholders : `{{key}}`. Trim whitespace inside braces.
- On missing variable → explicit error `FillError::MissingVariable`.
- No panic on expected errors.
- No external template engine dependency to keep audit surface minimal.
- Future: optional feature to accept `serde_json::Value` as variable source.
