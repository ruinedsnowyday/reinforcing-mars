use std::collections::HashMap;

/// Official tags in Terraforming Mars (excluding unofficial tags like Moon, Clone, Crime)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Tag {
    Building,
    Space,
    Science,
    Power,
    Earth,
    Jovian,
    Venus,  // From Venus Next expansion
    Plant,
    Microbe,
    Animal,
    City,
    Mars,
    Wild,
    Event,
}

impl Tag {
    /// Get all official tags
    pub fn all() -> Vec<Tag> {
        vec![
            Tag::Building,
            Tag::Space,
            Tag::Science,
            Tag::Power,
            Tag::Earth,
            Tag::Jovian,
            Tag::Venus,
            Tag::Plant,
            Tag::Microbe,
            Tag::Animal,
            Tag::City,
            Tag::Mars,
            Tag::Wild,
            Tag::Event,
        ]
    }
}

/// Tracks tag counts for a player
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Tags {
    counts: HashMap<Tag, u32>,
}

impl Tags {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    /// Add a tag (or multiple tags)
    pub fn add(&mut self, tag: Tag, count: u32) {
        *self.counts.entry(tag).or_insert(0) += count;
    }

    /// Remove a tag (or multiple tags)
    pub fn remove(&mut self, tag: Tag, count: u32) {
        if let Some(current) = self.counts.get_mut(&tag) {
            *current = current.saturating_sub(count);
            if *current == 0 {
                self.counts.remove(&tag);
            }
        }
    }

    /// Get the count of a specific tag
    /// WILD tags are automatically included when counting any tag (except when counting WILD itself)
    pub fn count(&self, tag: Tag, include_wild: bool) -> u32 {
        let base_count = self.counts.get(&tag).copied().unwrap_or(0);
        
        // Wild tags can substitute for any tag (except Wild itself)
        if include_wild && tag != Tag::Wild {
            let wild_count = self.counts.get(&Tag::Wild).copied().unwrap_or(0);
            base_count + wild_count
        } else {
            base_count
        }
    }

    /// Get the raw count of a tag (without WILD substitution)
    pub fn raw_count(&self, tag: Tag) -> u32 {
        self.counts.get(&tag).copied().unwrap_or(0)
    }

    /// Count all tags (total)
    pub fn total(&self) -> u32 {
        self.counts.values().sum()
    }

    /// Check if player has at least `required` of a tag
    /// By default, includes WILD tags as substitutes
    pub fn has(&self, tag: Tag, required: u32) -> bool {
        self.count(tag, true) >= required
    }

    /// Check if player has all required tags (for card requirements)
    /// WILD tags are automatically used as substitutes
    pub fn has_all(&self, required: &HashMap<Tag, u32>) -> bool {
        required.iter().all(|(tag, &count)| self.has(*tag, count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_operations() {
        let mut tags = Tags::new();
        
        // Test adding tags
        tags.add(Tag::Building, 3);
        tags.add(Tag::Space, 2);
        assert_eq!(tags.count(Tag::Building, true), 3);
        assert_eq!(tags.count(Tag::Space, true), 2);
        
        // Test removing tags
        tags.remove(Tag::Building, 1);
        assert_eq!(tags.count(Tag::Building, true), 2);
        
        // Test has
        assert!(tags.has(Tag::Building, 2));
        assert!(!tags.has(Tag::Building, 3));
        
        // Test total
        assert_eq!(tags.total(), 4); // 2 building + 2 space
    }

    #[test]
    fn test_wild_tag_substitution() {
        let mut tags = Tags::new();
        
        // Add some building tags and wild tags
        tags.add(Tag::Building, 2);
        tags.add(Tag::Wild, 1);
        
        // Wild should substitute for Building
        assert_eq!(tags.count(Tag::Building, true), 3); // 2 building + 1 wild
        assert_eq!(tags.count(Tag::Building, false), 2); // raw count without wild
        
        // Wild should also substitute for Space
        assert_eq!(tags.count(Tag::Space, true), 1); // 0 space + 1 wild
        assert_eq!(tags.count(Tag::Space, false), 0); // raw count
        
        // Counting Wild itself doesn't include substitution
        assert_eq!(tags.count(Tag::Wild, true), 1);
        assert_eq!(tags.count(Tag::Wild, false), 1);
    }
}

