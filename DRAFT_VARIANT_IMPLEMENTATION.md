# Draft and No-Draft Variant Implementation

## Official Rules Summary

### Draft Variant
- During Research phase, players draft 4 cards instead of just drawing 4
- Each player gets 4 cards and chooses one to draft, putting it aside and passing the rest to the next player
- Then you have 3 cards; set aside 1 and pass the rest to the left
- Then set aside 1 of the 2 you receive, pass the last card, and finally receive your last card
- Then examine the 4 cards you've set aside (drafted) and choose which to buy (3 M€ each) and which to discard
- **Drafting is NOT used during the first generation** (first Research phase is skipped)
- Cards are passed **clockwise during even-numbered generations** and **counter-clockwise during odd-numbered generations**

### No-Draft Variant
- Each player draws 4 cards directly
- Decides which to buy (0-4 cards)
- Each card costs 3 M€ to buy
- Selected cards are added to hand
- Unselected cards are discarded
- **This phase is skipped in generation 1** (setup phase handles initial card selection)

## Implementation Status

### ✅ Correctly Implemented

1. **Draft Variant Flag**: Added `draft_variant: bool` to `Game` struct
2. **Standard Draft Pass Direction**: 
   - Even generations (2, 4, 6...): Pass clockwise (After)
   - Odd generations (3, 5, 7...): Pass counter-clockwise (Before)
   - ✅ Matches official rules

3. **Standard Draft Flow**:
   - Players draft 4 cards through 4 rounds (1 card per round)
   - After drafting, cards remain in `drafted_cards` (not moved to hand)
   - Research phase then handles buying from `drafted_cards`
   - ✅ Matches official rules

4. **No-Draft Variant Flow**:
   - Cards are drawn directly to `drafted_cards` (4 cards per player)
   - Players select which to buy (3 M€ each)
   - Selected cards are added to hand
   - Unselected cards are discarded
   - ✅ Matches official rules

5. **Research Phase Handling**:
   - `start_standard_research_phase()` checks `draft_variant` flag
   - If draft variant: validates players have 4 cards in `drafted_cards` (from draft)
   - If no-draft variant: deals 4 cards directly to `drafted_cards`
   - ✅ Matches official rules

6. **Draft Not Used in Generation 1**:
   - Drafting is only used in generation 2+
   - Generation 1 uses initial research phase (different flow)
   - ✅ Matches official rules

### Implementation Details

#### Draft Variant Flow
1. **Draft Phase** (generation 2+):
   - `start_draft(DraftType::Standard)` draws 4 cards per player
   - Players draft 1 card per round for 4 rounds
   - Cards are passed based on generation (even = clockwise, odd = counter-clockwise)
   - After 4 rounds, all players have 4 cards in `drafted_cards`
   - `end_draft_iteration()` transitions to Research phase (cards stay in `drafted_cards`)

2. **Research Phase** (after draft):
   - `start_standard_research_phase()` validates players have 4 cards in `drafted_cards`
   - Players select which cards to buy (3 M€ each) via `select_project_cards()`
   - Selected cards are added to hand
   - Unselected cards are discarded

#### No-Draft Variant Flow
1. **Research Phase** (generation 2+):
   - `start_standard_research_phase()` deals 4 cards directly to `drafted_cards`
   - Players select which cards to buy (3 M€ each) via `select_project_cards()`
   - Selected cards are added to hand
   - Unselected cards are discarded

### Key Changes Made

1. **Added `draft_variant` flag to Game struct**
2. **Updated `start_standard_research_phase()`**:
   - Checks `draft_variant` flag
   - If draft variant: validates cards are in `drafted_cards` (from draft)
   - If no-draft variant: deals 4 cards to `drafted_cards`

3. **Fixed `end_draft_iteration()` for Standard draft**:
   - Cards remain in `drafted_cards` (not moved to hand)
   - Research phase handles moving selected cards to hand

4. **Pass Direction Logic** (already correct):
   - Even generations: `PassDirection::After` (clockwise)
   - Odd generations: `PassDirection::Before` (counter-clockwise)

## Summary

The implementation now correctly handles both draft and no-draft variants:
- **Draft variant**: Players draft 4 cards, then buy from those 4 cards
- **No-draft variant**: Players draw 4 cards directly, then buy from those 4 cards
- Both variants: Cards cost 3 M€ each, selected cards go to hand, unselected are discarded
- Pass direction alternates by generation (even = clockwise, odd = counter-clockwise)
- Drafting is not used in generation 1 (as per official rules)

