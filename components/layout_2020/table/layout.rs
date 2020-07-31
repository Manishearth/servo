use super::TableContainer;
use crate::context::LayoutContext;
use crate::formatting_contexts::IndependentLayout;
use crate::positioned::PositioningContext;
use crate::sizing::ContentSizes;
use crate::ContainingBlock;

impl TableContainer {
    pub fn inline_content_sizes(&self) -> ContentSizes {
        // FIXME
        ContentSizes::zero()
    }

    pub(crate) fn layout(
        &self,
        layout_context: &LayoutContext,
        positioning_context: &mut PositioningContext,
        containing_block: &ContainingBlock,
        tree_rank: usize,
    ) -> IndependentLayout {
        todo!()
    }
}
