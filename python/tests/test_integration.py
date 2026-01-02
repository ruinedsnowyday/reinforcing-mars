"""Integration tests for game flow"""
import pytest
from reinforcing_mars_sim import PyGame, PyAction, PyPayment, PyPaymentMethod


def test_full_game_flow():
    """Test a basic game flow"""
    game = PyGame.new(num_players=2, seed=12345)
    
    # Initial state
    assert game.get_generation() == 1
    assert not game.is_terminal()
    
    # Get observation
    obs = game.get_observation()
    assert obs is not None
    assert "players" in obs
    assert len(obs["players"]) == 2


def test_action_validation():
    """Test action validation"""
    game = PyGame.new(num_players=2, seed=12345)
    
    # Pass should always be valid (if in action phase)
    pass_action = PyAction("Pass")
    
    # Check if valid (may fail if not in action phase, which is expected)
    is_valid = game.is_action_valid(pass_action)
    assert isinstance(is_valid, bool)


def test_global_parameters():
    """Test getting global parameters"""
    game = PyGame.new(num_players=2, seed=12345)
    params = game.get_global_parameters()
    
    assert params is not None
    assert "oceans" in params
    assert "oxygen" in params
    assert "temperature" in params
    assert params["oceans"] >= 0
    assert params["oxygen"] >= 0


def test_player_data_consistency():
    """Test that player data is consistent between methods"""
    game = PyGame.new(num_players=2, seed=12345)
    
    # Get player directly
    player1 = game.get_player("Player 1")
    
    # Get observation
    obs = game.get_observation()
    obs_player1 = obs["players"][0]
    
    # Check consistency
    assert player1.id == obs_player1["id"]
    assert player1.terraform_rating == obs_player1["terraform_rating"]


def test_multiple_games():
    """Test creating multiple games with different seeds"""
    game1 = PyGame.new(num_players=2, seed=11111)
    game2 = PyGame.new(num_players=2, seed=22222)
    
    # Games should be independent
    assert game1.get_generation() == game2.get_generation()
    # But states might differ due to different seeds
    obs1 = game1.get_observation()
    obs2 = game2.get_observation()
    
    assert obs1 is not None
    assert obs2 is not None


def test_different_player_counts():
    """Test games with different player counts"""
    for num_players in [1, 2, 3, 4]:
        game = PyGame.new(num_players=num_players, seed=12345)
        players = game.get_players()
        assert len(players) == num_players


def test_game_options():
    """Test different game configuration options"""
    # Test with different board types
    for board_type in ["Tharsis", "Hellas", "Elysium"]:
        game = PyGame.new(
            num_players=2,
            seed=12345,
            board_type=board_type
        )
        assert game is not None
    
    # Test with expansions
    game = PyGame.new(
        num_players=2,
        seed=12345,
        corporate_era=True,
        venus_next=True,
        colonies=True,
        prelude=True,
        draft_variant=True
    )
    assert game is not None

