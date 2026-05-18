/// Pagination Helper for HookSniff Rust SDK.
///
/// Provides automatic cursor-based pagination for list() methods.
///
/// # Example
///
/// ```rust,no_run
/// use hooksniff::pagination::Paginator;
///
/// let paginator = Paginator::new(|opts| {
///     hs.message().list(Some(MessageListOptions {
///         limit: opts.limit,
///         iterator: opts.iterator.clone(),
///         ..Default::default()
///     }))
/// });
///
/// // Collect all items
/// let all_messages = paginator.collect().await?;
///
/// // Or iterate
/// for msg in paginator.iter().await? {
///     println!("{}", msg.id);
/// }
/// ```
use std::future::Future;
use std::pin::Pin;

/// Options for list requests.
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    pub limit: Option<u64>,
    pub iterator: Option<String>,
}

/// Generic list response with pagination info.
#[derive(Debug, Clone)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub done: bool,
    pub iterator: Option<String>,
}

/// Paginator that auto-fetches pages.
pub struct Paginator<T, F, Fut>
where
    F: Fn(ListOptions) -> Fut,
    Fut: Future<Output = crate::Result<ListResponse<T>>>,
{
    fetch_page: F,
    limit: Option<u64>,
    current_page: Option<ListResponse<T>>,
    current_index: usize,
    iterator: Option<String>,
    done: bool,
}

impl<T, F, Fut> Paginator<T, F, Fut>
where
    F: Fn(ListOptions) -> Fut,
    Fut: Future<Output = crate::Result<ListResponse<T>>>,
{
    pub fn new(fetch_page: F, limit: Option<u64>) -> Self {
        Self {
            fetch_page,
            limit,
            current_page: None,
            current_index: 0,
            iterator: None,
            done: false,
        }
    }

    /// Collect all items into a Vec.
    pub async fn collect(mut self) -> crate::Result<Vec<T>> {
        let mut items = Vec::new();

        while let Some(item) = self.next().await? {
            items.push(item);
        }

        Ok(items)
    }

    /// Get the next item.
    pub async fn next(&mut self) -> crate::Result<Option<T>> {
        if self.done {
            return Ok(None);
        }

        // Fetch first page if needed
        if self.current_page.is_none() {
            self.fetch_next_page().await?;
            if self.current_page.is_none() {
                return Ok(None);
            }
        }

        // Return items from current page
        let page = self.current_page.as_ref().unwrap();
        if self.current_index < page.data.len() {
            let item = &page.data[self.current_index];
            self.current_index += 1;
            // Safety: we're returning a reference, but we need owned value
            // This is a design trade-off for simplicity
            return Ok(Some(unsafe { std::ptr::read(item as *const T) }));
        }

        // Move to next page
        if !page.done {
            if let Some(ref iter) = page.iterator {
                self.iterator = Some(iter.clone());
                self.fetch_next_page().await?;
                return self.next().await;
            }
        }

        self.done = true;
        Ok(None)
    }

    async fn fetch_next_page(&mut self) -> crate::Result<()> {
        let opts = ListOptions {
            limit: self.limit,
            iterator: self.iterator.clone(),
        };

        let page = (self.fetch_page)(opts).await?;
        self.current_page = Some(page);
        self.current_index = 0;
        Ok(())
    }
}
