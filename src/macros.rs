//! Utility macros for kalshi-rs
//!
//! Provides macros for common patterns like paginated API endpoints.


/// This macro creates an async function that repeatedly calls a single-page endpoint,
/// updating the cursor between requests, until all data has been retrieved.
/// Returns a flattened vector of all items from the specified response field.
///
/// # Requirements
///
/// - The query type must implement `Clone` and have a `cursor: Option<String>` field
/// - The response type must have a `cursor: Option<String>` field
/// - The response type must have a field containing a `Vec` of items
#[macro_export]
macro_rules! paginated_endpoint {
    (
        $(#[$meta:meta])*
        pub async fn $fn_name:ident(
            &self,
            params: & $query_type:ty
        ) -> Result<Vec<$item_type:ty>, $error_type:ty> {
            single_page_fn: $single_page_fn:ident,
            response_field: $field:ident $(,)?
        }
    ) => {
        $(#[$meta])*
        pub async fn $fn_name(
            &self,
            params: &$query_type,
        ) -> Result<Vec<$item_type>, $error_type> {
            let mut all_items: Vec<$item_type> = Vec::new();
            let mut query = params.clone();

            loop {
                let response = self.$single_page_fn(&query).await?;
                let next_cursor = response.cursor.clone();
                all_items.extend(response.$field);

                match next_cursor {
                    Some(cursor) if !cursor.is_empty() => {
                        query.cursor = Some(cursor);
                    }
                    _ => break,
                }
            }

            Ok(all_items)
        }
    };
}
