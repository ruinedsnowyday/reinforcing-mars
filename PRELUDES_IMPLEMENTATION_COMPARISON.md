# Preludes Implementation Comparison

## Official Rules Summary

According to the official Prelude expansion rules:

1. **Dealing**: During setup, 4 Prelude cards are dealt to each player
2. **Selection**: Players choose 2 Prelude cards to keep at the same time as choosing corporations and project cards (step 6)
3. **Cost**: Prelude cards do NOT cost anything to keep (unlike project cards which cost 3 M€ each)
4. **Discarding**: The remaining 2 Prelude cards (not selected) are discarded
5. **Playing Order**: After all corporations have been played and cards paid for (step 7), there is an extra round (step 7b) where each player plays their pair of picked Prelude cards in player order
6. **Prelude Cards Stay in Play**: They work like green cards and stay in play with their tags visible

## Current Implementation Status

### ✅ Correctly Implemented

1. **Dealing 4 Preludes**: 
   - `start_initial_research_phase()` now deals 4 prelude cards to each player in `dealt_prelude_cards`
   - ✅ Matches official rules

2. **Selection from Dealt Cards**:
   - `select_preludes()` validates that selected preludes are in `dealt_prelude_cards`
   - ✅ Matches official rules

3. **No Cost for Preludes**:
   - Prelude selection does not deduct megacredits (unlike project cards)
   - ✅ Matches official rules

4. **Discarding Unselected Preludes**:
   - When 2 preludes are selected, the remaining 2 are automatically removed from `dealt_prelude_cards`
   - ✅ Matches official rules

5. **Phase Transition**:
   - After research phase completes, if preludes are enabled, game transitions to PRELUDES phase
   - Preludes are played after corporations are selected (which applies starting resources)
   - ✅ Matches official rules

6. **Prelude Playing**:
   - `play_prelude()` validates prelude is in selected preludes
   - `has_played_all_preludes()` tracks if player has played both preludes
   - `advance_prelude_turn()` moves to next player after current player finishes
   - ✅ Matches official rules

### ⚠️ Placeholder / To Be Implemented

1. **Prelude Effects**:
   - `execute_prelude_effects()` is currently a placeholder
   - Will be expanded when card system is implemented (Phase 5)
   - ⚠️ Expected - card system not yet implemented

2. **Prelude Cards Stay in Play**:
   - Preludes are added to `played_cards` when played
   - Tags from preludes are not yet tracked (will be implemented with card system)
   - ⚠️ Expected - card system not yet implemented

## Differences from Previous Implementation

### Fixed Issues

1. **Before**: Preludes were checked in `cards_in_hand` (incorrect)
   - **After**: Preludes are checked in `dealt_prelude_cards` (correct)

2. **Before**: Preludes were not dealt during initial research phase
   - **After**: 4 preludes are dealt to each player during `start_initial_research_phase()`

3. **Before**: Unselected preludes were not discarded
   - **After**: Unselected preludes are automatically removed from `dealt_prelude_cards`

## Test Coverage

All tests updated and passing:
- ✅ `test_prelude_selection`: Verifies 4 preludes are dealt, 2 are selected, 2 are discarded
- ✅ `test_prelude_selection_wrong_count`: Verifies error when selecting wrong number
- ✅ `test_research_phase_with_preludes`: Verifies full research phase with preludes
- ✅ All prelude playing tests in `preludes.rs`

## Conclusion

The current implementation correctly follows the official Prelude expansion rules for:
- Dealing 4 preludes per player
- Selecting 2 preludes (no cost)
- Discarding remaining 2 preludes
- Playing preludes in player order after corporations are selected
- Phase transitions

The only remaining work is implementing actual prelude card effects, which is expected to be done in Phase 5 when the full card system is implemented.

