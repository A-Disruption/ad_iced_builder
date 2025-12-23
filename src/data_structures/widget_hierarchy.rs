use std::collections::HashSet;
use iced::{Length, Padding};

use crate::data_structures::properties::messages::*;
use crate::data_structures::properties::properties::CommonProperties;
use crate::data_structures::types::types::*;
use crate::enum_builder::TypeSystem;


/// Central widget hierarchy manager - Simplified to use only IDs
// #[derive(Debug, Clone,)]
pub struct WidgetHierarchy {
    root: Widget,
    selected_ids: HashSet<WidgetId>,
    next_id: usize,
    pub common_properties: Option<CommonProperties>,
}

impl WidgetHierarchy {
    pub fn new(root_type: WidgetType) -> Self {
        let mut selected_ids = HashSet::new();
        selected_ids.insert(WidgetId(0)); // Start with root selected

        Self {
            root: Widget::new(root_type, WidgetId(0)),
            selected_ids,
            next_id: 1,
            common_properties: None
        }
    }
    
    pub fn root(&self) -> &Widget {
        &self.root
    }
    
    pub fn selected_ids(&self) -> &HashSet<WidgetId> {
        &self.selected_ids
    }

    pub fn set_selected_ids(&mut self, ids: HashSet<WidgetId>) {
        // Filter to only valid IDs
        self.selected_ids = ids.into_iter()
            .filter(|id| self.widget_exists(*id))
            .collect();
        
        self.common_properties = Some(self.get_common_properties());
    }
    
    pub fn get_single_selected(&self) -> Option<&Widget> {
        if self.selected_ids.len() == 1 {
            let id = self.selected_ids.iter().next()?;
            self.get_widget_by_id(*id)
        } else {
            None
        }
    }
    
    pub fn get_widget_by_id(&self, id: WidgetId) -> Option<&Widget> {
        fn find_widget(widget: &Widget, target_id: WidgetId) -> Option<&Widget> {
            if widget.id == target_id {
                return Some(widget);
            }
            for child in &widget.children {
                if let Some(found) = find_widget(child, target_id) {
                    return Some(found);
                }
            }
            None
        }
        find_widget(&self.root, id)
    }
    
    pub fn get_widget_by_id_mut(&mut self, id: WidgetId) -> Option<&mut Widget> {
        fn find_widget_mut(widget: &mut Widget, target_id: WidgetId) -> Option<&mut Widget> {
            if widget.id == target_id {
                return Some(widget);
            }
            for child in &mut widget.children {
                if let Some(found) = find_widget_mut(child, target_id) {
                    return Some(found);
                }
            }
            None
        }
        find_widget_mut(&mut self.root, id)
    }
    
    pub fn widget_exists(&self, id: WidgetId) -> bool {
        self.get_widget_by_id(id).is_some()
    }

    pub fn can_add_child(&self, parent_id: WidgetId, widget_type: WidgetType) -> bool {
        if let Some(parent) = self.get_widget_by_id(parent_id) {
            if !can_have_children(&parent.widget_type) { return false; }

            if parent_id == self.root.id {
                return parent.children.is_empty()
                    && matches!(widget_type, WidgetType::Column | WidgetType::Row);
            }

            match parent.widget_type {
                WidgetType::Scrollable => {
                    if !parent.children.is_empty() { return false; }
                    matches!(widget_type, WidgetType::Column | WidgetType::Row | WidgetType::Container)
                }
                WidgetType::Container => parent.children.is_empty(),
                WidgetType::Tooltip   => parent.children.len() < 2, // <= 2 children
                WidgetType::MouseArea => parent.children.is_empty(),
                _ => true,
            }
        } else { false }
    }
    
    pub fn add_child(&mut self, parent_id: WidgetId, widget_type: WidgetType) -> Result<WidgetId, String> {
        if !self.can_add_child(parent_id, widget_type) {
            if parent_id == self.root.id {
                if self.root.children.is_empty() {
                    return Err("Root container can only have Column or Row as its first child".to_string());
                } else {
                    return Err("Root container can only have one child".to_string());
                }
            } else {
                return Err(format!("Cannot add {:?} to this parent", widget_type));
            }
        }

        let child_id = WidgetId(self.next_id);
        self.next_id += 1;
        let mut child = Widget::new(widget_type, child_id);

        // Check if parent is under a scrollable and handle accordingly
        if let Some((_, scroll_dir)) = self.get_scrollable_ancestor_info(parent_id) {
            let should_block_height = match scroll_dir {
                iced::widget::scrollable::Direction::Vertical(_) => true,
                iced::widget::scrollable::Direction::Both { .. } => true,
                iced::widget::scrollable::Direction::Horizontal(_) => false,
            };
            
            let should_block_width = match scroll_dir {
                iced::widget::scrollable::Direction::Horizontal(_) => true,
                iced::widget::scrollable::Direction::Both { .. } => true,
                iced::widget::scrollable::Direction::Vertical(_) => false,
            };
            
            if should_block_height {
                let orig = child.properties.height;
                if matches!(orig, Length::Fill | Length::FillPortion(_)) {
                    child.properties.saved_height_before_scrollable = Some(orig);
                    child.properties.height = Length::Shrink;
                }
            }
            
            if should_block_width {
                let orig = child.properties.width;
                if matches!(orig, Length::Fill | Length::FillPortion(_)) {
                    child.properties.saved_width_before_scrollable = Some(orig);
                    child.properties.width = Length::Shrink;
                }
            }
        }

        if let Some(parent) = self.get_widget_by_id_mut(parent_id) {
            parent.children.push(child);
            Ok(child_id)
        } else {
            Err("Parent widget not found".to_string())
        }
    }
    
    pub fn delete_widget(&mut self, id: WidgetId) -> Result<(), String> {
        if id == self.root.id {
            return Err("Cannot delete root widget".to_string());
        }
        
        if let Some(parent_id) = self.find_parent_id(id) {
            if let Some(parent) = self.get_widget_by_id_mut(parent_id) {
                parent.children.retain(|child| child.id != id);
                
                // Remove from selection
                self.selected_ids.remove(&id);
                
                // If nothing selected, select parent
                if self.selected_ids.is_empty() {
                    self.selected_ids.insert(parent_id);
                }
                
                Ok(())
            } else {
                Err("Parent widget not found".to_string())
            }
        } else {
            Err("Cannot find parent of widget".to_string())
        }
    }
    
    pub fn find_parent_id(&self, child_id: WidgetId) -> Option<WidgetId> {
        fn find_parent(widget: &Widget, target_id: WidgetId) -> Option<WidgetId> {
            for child in &widget.children {
                if child.id == target_id {
                    return Some(widget.id);
                }
                if let Some(parent_id) = find_parent(child, target_id) {
                    return Some(parent_id);
                }
            }
            None
        }
        find_parent(&self.root, child_id)
    }

    pub fn apply_property_change(&mut self, id: WidgetId, change: PropertyChange, type_system: &TypeSystem) {
        // Special handling for scrollable direction changes
        if let PropertyChange::ScrollableDirection(new_dir) = change.clone() {
            if let Some(widget) = self.get_widget_by_id_mut(id) {
                widget.properties.scroll_dir = new_dir;
            }
            // Re-sanitize the subtree with new direction
            self.sanitize_subtree_for_scrollable(id);
            return;
        }
        
        // Handle height changes under scrollable
        if let PropertyChange::Height(h) = change.clone() {
            if let Some((_, scroll_dir)) = self.get_scrollable_ancestor_info(id) {
                let should_block = match scroll_dir {
                    iced::widget::scrollable::Direction::Vertical(_) => true,
                    iced::widget::scrollable::Direction::Both { .. } => true,
                    iced::widget::scrollable::Direction::Horizontal(_) => false,
                };
                
                if should_block && matches!(h, Length::Fill | Length::FillPortion(_)) {
                    if let Some(w) = self.get_widget_by_id_mut(id) {
                        if w.properties.saved_height_before_scrollable.is_none() {
                            w.properties.saved_height_before_scrollable = Some(h);
                        }
                        w.properties.height = Length::Shrink;
                    }
                    return;
                }
            }
        }
        
        // Handle width changes under scrollable
        if let PropertyChange::Width(w) = change.clone() {
            if let Some((_, scroll_dir)) = self.get_scrollable_ancestor_info(id) {
                let should_block = match scroll_dir {
                    iced::widget::scrollable::Direction::Horizontal(_) => true,
                    iced::widget::scrollable::Direction::Both { .. } => true,
                    iced::widget::scrollable::Direction::Vertical(_) => false,
                };
                
                if should_block && matches!(w, Length::Fill | Length::FillPortion(_)) {
                    if let Some(widget) = self.get_widget_by_id_mut(id) {
                        if widget.properties.saved_width_before_scrollable.is_none() {
                            widget.properties.saved_width_before_scrollable = Some(w);
                        }
                        widget.properties.width = Length::Shrink;
                    }
                    return;
                }
            }
        }
        
        if let Some(widget) = self.get_widget_by_id_mut(id) {
            apply_property_change(&mut widget.properties, change, type_system);
        }
    }

    pub fn move_widget(
        &mut self,
        id: WidgetId,
        new_parent_id: WidgetId,
        mut new_index: usize,
    ) -> Result<(), String> {
        if id == self.root.id {
            return Err("Cannot move root widget".into());
        }
        if !self.widget_exists(id) {
            return Err("Widget to move not found".into());
        }
        if !self.widget_exists(new_parent_id) {
            return Err("New parent not found".into());
        }

        // Prevent cycles: cannot move a node into its own subtree
        if self.is_descendant(id, new_parent_id) {
            return Err("Cannot move a widget into its own descendant".into());
        }

        // Parent capability checks
        let new_parent_ty = self.get_widget_by_id(new_parent_id).unwrap().widget_type;
        if !can_have_children(&new_parent_ty) {
            return Err(format!("{new_parent_ty:?} cannot have children"));
        }

        if matches!(new_parent_ty, WidgetType::Tooltip) {
            let count = self.get_widget_by_id(new_parent_id).unwrap().children.len();
            if count >= 2 && self.find_parent_id(id) != Some(new_parent_id) {
                return Err("Tooltip can only contain two children".into());
            }
        }

        if matches!(new_parent_ty, WidgetType::MouseArea) {
            let count = self.get_widget_by_id(new_parent_id).unwrap().children.len();
            if count >= 1 && self.find_parent_id(id) != Some(new_parent_id) {
                return Err("MouseArea can only contain one child".into());
            }
        }

        // Root container constraints
        if new_parent_id == self.root.id {
            // Only Column/Row allowed under root
            let moving_ty = self.get_widget_by_id(id).unwrap().widget_type;
            if !matches!(moving_ty, WidgetType::Column | WidgetType::Row) {
                return Err("Root can only contain Column or Row".into());
            }
            // Root can have only one child (unless we're reordering the same one)
            let root_children = &self.root.children;
            let already_under_root = self.find_parent_id(id) == Some(self.root.id);
            if !already_under_root && !root_children.is_empty() {
                return Err("Root container can only have one child".into());
            }
            // Clamp index for root (0 or existing 0)
            new_index = 0;
        }

        // Detach node from current parent
        let old_parent_id = self.find_parent_id(id).ok_or("Old parent not found")?;
        let node = self.remove_and_return(id).ok_or("Failed to detach node")?;

        // If moving within the same parent and we removed a lower index, fix target index
        if old_parent_id == new_parent_id {
            let siblings_len = self.get_widget_by_id(new_parent_id).unwrap().children.len();
            // After removal, children length decreased by 1. Clamp index accordingly.
            new_index = new_index.min(siblings_len);
        } else {
            let siblings_len = self.get_widget_by_id(new_parent_id).unwrap().children.len();
            new_index = new_index.min(siblings_len);
        }

        // Insert into new parent
        let parent = self.get_widget_by_id_mut(new_parent_id).ok_or("New parent not found")?;
        parent.children.insert(new_index, node);

        Ok(())
    }

    fn is_descendant(&self, ancestor: WidgetId, candidate: WidgetId) -> bool {
        fn walk(w: &Widget, anc: WidgetId, cand: WidgetId) -> bool {
            if w.id == anc {
                return contains(&w.children, cand);
            }
            for c in &w.children {
                if walk(c, anc, cand) {
                    return true;
                }
            }
            false
        }
        fn contains(children: &[Widget], id: WidgetId) -> bool {
            for c in children {
                if c.id == id { return true; }
                if contains(&c.children, id) { return true; }
            }
            false
        }
        walk(&self.root, ancestor, candidate)
    }

    /// Remove a node from the tree and return it.
    fn remove_and_return(&mut self, id: WidgetId) -> Option<Widget> {
        fn take_from(parent: &mut Widget, id: WidgetId) -> Option<Widget> {
            if let Some(pos) = parent.children.iter().position(|c| c.id == id) {
                return Some(parent.children.remove(pos));
            }
            for c in &mut parent.children {
                if let Some(found) = take_from(c, id) {
                    return Some(found);
                }
            }
            None
        }
        if id == self.root.id { return None; }
        take_from(&mut self.root, id)
    }

    /// Toggle Row<->Column and Container<->Scrollable without resetting props/children
    pub fn swap_kind(&mut self, id: WidgetId) {
        let old_type;
        {
            let w = match self.get_widget_by_id(id) { Some(w) => w, None => return };
            old_type = w.widget_type;
        }

        if let Some(w) = self.get_widget_by_id_mut(id) {
            let new_type = match w.widget_type {
                WidgetType::Row        => WidgetType::Column,
                WidgetType::Column     => WidgetType::Row,
                WidgetType::Container  => WidgetType::Scrollable,
                WidgetType::Scrollable => WidgetType::Container,
                _ => w.widget_type,
            };

            if new_type != w.widget_type {
                w.widget_type = new_type;
                w.name = format!("{:?}", w.widget_type);
            }
        }

        // If we just became a Scrollable, clamp subtree.
        if let Some(w) = self.get_widget_by_id(id) {
            if matches!(w.widget_type, WidgetType::Scrollable) {
                self.sanitize_subtree_for_scrollable(id);
                return;
            }
        }

        // If we were a Scrollable and swapped back to Container, restore subtree.
        if matches!(old_type, WidgetType::Scrollable) {
            self.restore_subtree_after_scrollable(id);
        }
    }

    // Get the scrollable direction of the nearest scrollable ancestor (if any)
    pub fn get_scrollable_ancestor_info(&self, mut id: WidgetId) -> Option<(WidgetId, iced::widget::scrollable::Direction)> {
        while let Some(parent_id) = self.find_parent_id(id) {
            if let Some(parent) = self.get_widget_by_id(parent_id) {
                if matches!(parent.widget_type, WidgetType::Scrollable) {
                    return Some((parent_id, parent.properties.scroll_dir));
                }
                id = parent_id;
            } else {
                break;
            }
        }
        None
    }

    // Force all descendants of a Scrollable to NOT fill vertically
    fn sanitize_subtree_for_scrollable(&mut self, root_scrollable_id: WidgetId) {
        // Get the scrollable's direction
        let scroll_dir = if let Some(scrollable) = self.get_widget_by_id(root_scrollable_id) {
            scrollable.properties.scroll_dir
        } else {
            return;
        };
        
        fn clamp_descendants(widget: &mut Widget, scroll_dir: iced::widget::scrollable::Direction) {
            let should_block_height = match scroll_dir {
                iced::widget::scrollable::Direction::Vertical(_) => true,
                iced::widget::scrollable::Direction::Both { .. } => true,
                iced::widget::scrollable::Direction::Horizontal(_) => false,
            };
            
            let should_block_width = match scroll_dir {
                iced::widget::scrollable::Direction::Horizontal(_) => true,
                iced::widget::scrollable::Direction::Both { .. } => true,
                iced::widget::scrollable::Direction::Vertical(_) => false,
            };
            
            // Handle height
            if should_block_height {
                match widget.properties.height {
                    Length::Fill | Length::FillPortion(_) => {
                        if widget.properties.saved_height_before_scrollable.is_none() {
                            widget.properties.saved_height_before_scrollable = Some(widget.properties.height);
                        }
                        widget.properties.height = Length::Shrink;
                    }
                    _ => {}
                }
            } else {
                // Restore height if we're not blocking it anymore
                if let Some(h) = widget.properties.saved_height_before_scrollable.take() {
                    widget.properties.height = h;
                }
            }
            
            // Handle width
            if should_block_width {
                match widget.properties.width {
                    Length::Fill | Length::FillPortion(_) => {
                        if widget.properties.saved_width_before_scrollable.is_none() {
                            widget.properties.saved_width_before_scrollable = Some(widget.properties.width);
                        }
                        widget.properties.width = Length::Shrink;
                    }
                    _ => {}
                }
            } else {
                // Restore width if we're not blocking it anymore
                if let Some(w) = widget.properties.saved_width_before_scrollable.take() {
                    widget.properties.width = w;
                }
            }
            
            // Recurse to children
            for child in &mut widget.children {
                clamp_descendants(child, scroll_dir);
            }
        }
        
        if let Some(scrollable) = self.get_widget_by_id_mut(root_scrollable_id) {
            for child in &mut scrollable.children {
                clamp_descendants(child, scroll_dir);
            }
        }
    }

    // Restore any saved heights or widths after leaving a Scrollable subtree, or when a Scrollable Direction is changed.
    fn restore_subtree_after_scrollable(&mut self, root_container_id: WidgetId) {
        fn restore(widget: &mut Widget) {
            // Restore both saved dimensions
            if let Some(h) = widget.properties.saved_height_before_scrollable.take() {
                widget.properties.height = h;
            }
            if let Some(w) = widget.properties.saved_width_before_scrollable.take() {
                widget.properties.width = w;
            }
            for child in &mut widget.children {
                restore(child);
            }
        }
        if let Some(container) = self.get_widget_by_id_mut(root_container_id) {
            for child in &mut container.children {
                restore(child);
            }
        }
    }

    /// Validates that all selected widgets can be wrapped together
    pub fn validate_wrapping(&self) -> Result<WidgetId, String> {
        // Need at least one widget selected
        if self.selected_ids.is_empty() {
            return Err("No widgets selected".to_string());
        }
        
        // Can't wrap root
        if self.selected_ids.contains(&self.root.id) {
            return Err("Cannot wrap root widget".to_string());
        }
        
        // All selected widgets must share the same parent
        let parent_ids: std::collections::HashSet<_> = self.selected_ids
            .iter()
            .filter_map(|&id| self.find_parent_id(id))
            .collect();
            
        if parent_ids.len() != 1 {
            return Err("Selected widgets must have the same parent".to_string());
        }
        
        let parent_id = *parent_ids.iter().next().unwrap();
        Ok(parent_id)
    }

    /// Wraps selected widgets in a new container
    pub fn wrap_selected_in_container(
        &mut self, 
        container_type: WidgetType
    ) -> Result<WidgetId, String> {
        // Validate before making any changes
        let parent_id = self.validate_wrapping()?;
        
        // Validation for tooltip - takes 2 children
        if container_type == WidgetType::Tooltip && self.selected_ids.len() != 2 {
            return Err("Tooltip takes 2 widgets selected".to_string());
        }

        // Validation for MouseArea - takes 1 child
        if container_type == WidgetType::MouseArea && self.selected_ids.len() != 1 {
            return Err("MouseArea takes 1 widget selected".to_string());
        }
        
        // CRITICAL: Extract data from self before taking mutable borrow
        let selected_ids = self.selected_ids.clone();
        let wrapper_id = WidgetId(self.next_id);
        self.next_id += 1;
        
        let parent = self.get_widget_by_id_mut(parent_id)
            .ok_or("Parent not found")?;
        
        // Find indices of selected widgets in parent's children
        let mut selected_indices: Vec<usize> = parent.children
            .iter()
            .enumerate()
            .filter(|(_, child)| selected_ids.contains(&child.id)) // Use cloned set
            .map(|(i, _)| i)
            .collect();
        
        if selected_indices.is_empty() {
            return Err("No valid widgets to wrap".to_string());
        }
        
        selected_indices.sort_unstable();  // Ensure consistent order
        
        // Extract the selected widgets (in order)
        let first_index = selected_indices[0];
        let mut widgets_to_wrap = Vec::new();
        
        // Remove in reverse order to maintain indices
        for &idx in selected_indices.iter().rev() {
            let widget = parent.children.remove(idx);
            widgets_to_wrap.push(widget);
        }
        widgets_to_wrap.reverse();
        
        let mut wrapper = Widget::new(container_type, wrapper_id);
        wrapper.children = widgets_to_wrap;
        
        parent.children.insert(first_index, wrapper);
        
        self.selected_ids.clear();
        self.selected_ids.insert(wrapper_id);
        
        Ok(wrapper_id)
    }
    
    /// Gets all widgets that are currently selected
    pub fn get_selected_widgets(&self) -> Vec<&Widget> {
        self.selected_ids
            .iter()
            .filter_map(|&id| self.get_widget_by_id(id))
            .collect()
    }
    
    /// Finds properties that are common across all selected widgets
    pub fn get_common_properties(&self) -> CommonProperties {
        let selected = self.get_selected_widgets();
        
        if selected.is_empty() {
            return CommonProperties::default();
        }
        
        // Start with all possible properties as "common"
        // Then eliminate any that aren't shared by ALL selected widgets
        CommonProperties::from_widgets(&selected)
    }
    
    /// Applies a property change to all currently selected widgets
    pub fn apply_property_to_all_selected(
        &mut self, 
        change: PropertyChange,
        type_system: &TypeSystem
    ) {
        // Clone the selected IDs to avoid borrow checker issues
        let selected_ids: Vec<WidgetId> = self.selected_ids.iter().copied().collect();
        
        for widget_id in selected_ids {
            self.apply_property_change(widget_id, change.clone(), type_system);
        }

        match change {
            // Height / Width
            PropertyChange::Height(length) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.uniform_height = Some(length);
                }
            }
            PropertyChange::Width(width) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.uniform_width = Some(width);
                }
            }
            PropertyChange::DraftFixedHeight(height)=> {
                if let Some(prop) = &mut self.common_properties {
                    prop.draft_fixed_height = height;
                }
            }
            PropertyChange::DraftFillPortionHeight(height) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.draft_fill_portion_height = height;
                }
            }
            PropertyChange::DraftFixedWidth(width) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.draft_fixed_width = width;
                }
            }
            PropertyChange::DraftFillPortionWidth(width) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.draft_fill_portion_width = width;
                }
            }

            //Spacing
            PropertyChange::Spacing(spacing) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.uniform_spacing = Some(spacing);
                }
            }

            // Padding
            PropertyChange::PaddingMode(mode) => {
                if let Some(prop) = &mut self.common_properties {
                    prop.uniform_padding_mode = Some(mode);
                }
            }
            PropertyChange::PaddingUniform(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        *padding = Padding::new(v);
                    }
                }
            }
            PropertyChange::PaddingVertical(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.top = v;
                        padding.bottom = v;
                    }
                }
            }
            PropertyChange::PaddingHorizontal(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.left = v;
                        padding.right = v;
                    }
                }
            }
            PropertyChange::PaddingTop(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.top = v;
                    }
                }
            }
            PropertyChange::PaddingBottom(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.bottom = v;
                    }
                }
            }
            PropertyChange::PaddingLeft(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.left = v;
                    }
                }
            }
            PropertyChange::PaddingRight(v) => {
                if let Some(prop) = &mut self.common_properties {
                    if let Some(padding) = &mut prop.uniform_padding {
                        padding.right = v;
                    }
                }
            }
            _ => {}
        }

    }

}

fn can_have_children(widget_type: &WidgetType) -> bool {
    matches!(
        widget_type,
        WidgetType::Container | WidgetType::Row | WidgetType::Column | 
        WidgetType::Scrollable | WidgetType::Tooltip | 
        WidgetType::Stack | WidgetType::Themer | WidgetType::MouseArea
    )
}