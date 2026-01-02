"""Basic tests for Game wrapper"""
import pytest
from reinforcing_mars_sim import PyGame, PyAction, PyPayment, PyPaymentMethod


def test_game_creation():
    """Test creating a new game"""
    game = PyGame.new(num_players=2, seed=12345)
    assert game is not None
    assert game.get_generation() == 1
    assert game.get_phase() in ["InitialDrafting", "Research", "Action", "Production", "Solar", "Intergeneration", "End"]


def test_game_creation_with_options():
    """Test creating a game with various options"""
    game = PyGame.new(
        num_players=3,
        seed=54321,
        board_type="Hellas",
        corporate_era=True,
        venus_next=False,
        draft_variant=True
    )
    assert game is not None
    assert game.get_generation() == 1


def test_get_observation():
    """Test getting game observation"""
    game = PyGame.new(num_players=2, seed=12345)
    obs = game.get_observation()
    
    assert obs is not None
    assert "phase" in obs
    assert "generation" in obs
    assert "players" in obs
    assert "global_parameters" in obs
    assert len(obs["players"]) == 2


def test_get_players():
    """Test getting all players"""
    game = PyGame.new(num_players=2, seed=12345)
    players = game.get_players()
    
    assert players is not None
    assert len(players) == 2
    assert players[0].id == "Player 1"
    assert players[1].id == "Player 2"


def test_get_player_by_id():
    """Test getting a specific player"""
    game = PyGame.new(num_players=2, seed=12345)
    player = game.get_player("Player 1")
    
    assert player is not None
    assert player.id == "Player 1"
    assert player.name == "Player 1"


def test_get_player_invalid_id():
    """Test getting a player with invalid ID"""
    game = PyGame.new(num_players=2, seed=12345)
    
    with pytest.raises(Exception):  # Should raise ValueError
        game.get_player("Invalid Player")


def test_is_terminal():
    """Test checking if game is terminal"""
    game = PyGame.new(num_players=2, seed=12345)
    # Game should not be terminal at start
    assert game.is_terminal() == False


def test_get_reward():
    """Test getting reward for a player"""
    game = PyGame.new(num_players=2, seed=12345)
    reward = game.get_reward("Player 1")
    
    # Reward should be a float (victory points)
    assert isinstance(reward, float)
    assert reward >= 0


def test_get_valid_actions():
    """Test getting valid actions"""
    game = PyGame.new(num_players=2, seed=12345)
    actions = game.get_valid_actions()
    
    assert actions is not None
    assert len(actions) > 0
    # Should at least have Pass action
    assert any(action.action_type == "Pass" for action in actions)


def test_step_with_pass():
    """Test executing a Pass action"""
    game = PyGame.new(num_players=2, seed=12345)
    
    # Create a Pass action
    action = PyAction("Pass")
    
    # Execute the action
    result = game.step(action)
    
    assert result is not None
    assert "phase" in result
    assert "generation" in result

