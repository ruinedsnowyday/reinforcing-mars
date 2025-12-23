# Research Phase Implementation Comparison

## Official Rules Summary

According to the official rulebook:

**Standard Research Phase (Generation 2+):**
1. Each player draws 4 cards
2. Players "put down their hand cards" (hand is separate from the 4 drawn cards)
3. Decides which of the 4 drawn cards to buy (0-4 cards)
4. Each card costs 3 M€ to buy
5. Selected cards are added to hand
6. Unselected cards are discarded face down
7. No hand limit
8. **This phase is skipped in generation 1** (setup phase handles initial card selection)

**Example from rules:**
> "In the second generation, the players have their first normal research phase. They put down their hand cards and draw 4 cards each to buy from. Stanley decides to buy 3 of his cards, so he pays 9 M€ and discards the remaining card, adding the 3 new cards to his hand."

## Current Implementation Issues

### ❌ Issue 1: Cards Not Drawn to Separate Area
**Current**: Cards are placed directly in `cards_in_hand` during `start_standard_research_phase()`
```rust
player.cards_in_hand = (0..4).map(|i| format!("project_card_{i}")).collect();
```

**Problem**: According to rules, players should "put down their hand cards" first, meaning:
- Existing hand cards should remain in hand
- 4 new cards should be drawn to a separate area (like `drafted_cards` or a new field)
- Players select from the 4 drawn cards
- Selected cards are added to hand
- Unselected cards are discarded

**Should be**: Draw 4 cards to a separate area (e.g., `drafted_cards` or `research_draft_cards`), not directly to hand.

### ❌ Issue 2: Cost Only Applied in Generation 1
**Current**: 
```rust
// In initial research phase, player pays 3 M€ per card
if is_generation_1 {
    let cost = (card_ids.len() as u32) * 3;
    player.resources.subtract(...);
}
```

**Problem**: According to rules, **all research phases** (generation 2+) should charge 3 M€ per card. Generation 1 is different (it's part of setup, not standard research phase).

**Should be**: Charge 3 M€ per card in standard research phase (generation 2+), not just generation 1.

### ❌ Issue 3: Hand Cards Not Preserved
**Current**: When dealing cards, if `cards_in_hand.is_empty()`, cards are added. But the rules say players should "put down their hand cards" - meaning existing hand cards should be preserved, and the 4 drawn cards are separate.

**Problem**: The current implementation doesn't properly separate "hand cards" from "drawn cards to choose from".

**Should be**: 
- Preserve existing `cards_in_hand`
- Draw 4 cards to a separate area (e.g., `drafted_cards`)
- Player selects from the 4 drawn cards
- Selected cards are added to `cards_in_hand`
- Unselected cards are discarded

### ✅ Correctly Implemented

1. **Generation 1 Handling**: Research phase is correctly skipped in generation 1 (initial research phase is different)
2. **0-4 Cards Selection**: Players can select 0-4 cards (enforced by validation)
3. **Discarding Unselected Cards**: Unselected cards are removed (though they should be in a separate area first)

## Recommended Fixes

### Fix 1: Use `drafted_cards` for Research Phase
The `Player` struct already has a `drafted_cards` field. Use it for the 4 drawn cards in research phase:

```rust
// In start_standard_research_phase():
for player in &mut self.players {
    // Draw 4 cards to drafted_cards (separate from hand)
    player.drafted_cards = (0..4)
        .map(|i| format!("project_card_{i}"))
        .collect();
}
```

### Fix 2: Update `select_project_cards` for Standard Research
```rust
pub fn select_project_cards(&mut self, player_id: &PlayerId, card_ids: Vec<String>) -> Result<(), String> {
    // ...
    
    // For standard research (generation 2+), cards come from drafted_cards
    if self.generation > 1 {
        // Validate cards are in drafted_cards
        for card_id in &card_ids {
            if !player.drafted_cards.contains(card_id) {
                return Err(format!("Card {card_id} not in drafted cards"));
            }
        }
        
        // Add selected cards to hand
        player.cards_in_hand.extend(card_ids.clone());
        
        // Remove selected cards from drafted_cards (discard unselected ones)
        player.drafted_cards.retain(|c| !card_ids.contains(c));
        
        // Charge 3 M€ per card
        let cost = (card_ids.len() as u32) * 3;
        player.resources.subtract(Resource::Megacredits, cost);
    } else {
        // Generation 1: different logic (from cards_in_hand, already implemented)
        // ...
    }
}
```

### Fix 3: Update `start_standard_research_phase`
```rust
fn start_standard_research_phase(&mut self) -> Result<(), String> {
    // Check if draft variant is enabled
    // If draft variant, cards come from drafting (already in drafted_cards)
    // Otherwise, deal 4 cards to drafted_cards
    
    for player in &mut self.players {
        if self.draft_variant {
            // Cards should already be in drafted_cards from drafting phase
            // Just verify they have cards
            if player.drafted_cards.is_empty() {
                return Err(format!("Player {} has no drafted cards", player.id));
            }
        } else {
            // Deal 4 cards to drafted_cards (not to hand)
            player.drafted_cards = (0..4)
                .map(|i| format!("project_card_{i}"))
                .collect();
        }
    }
    
    Ok(())
}
```

## Summary

The main issues are:
1. **Cards should be drawn to a separate area** (`drafted_cards`), not directly to hand
2. **Cost should be applied in all research phases** (generation 2+), not just generation 1
3. **Hand cards should be preserved** - the 4 drawn cards are separate from existing hand

The current implementation mixes up the "hand" and "drawn cards to choose from" concepts, which doesn't match the official rules.

