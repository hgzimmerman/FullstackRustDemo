pub mod author_markdown_toggle;

// Currently can't use this because the vtag property isn't public, which this relies upon.
// To fix this, I need to:
// * update yew to the current commit hash
// * fork yew
// * make vtag enum variant public
// * fix any other errors in the markdown rendering code caused by updating.
pub mod markdown;

