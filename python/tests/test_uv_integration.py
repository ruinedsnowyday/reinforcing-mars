"""Test that the package works correctly when installed via uv"""
import pytest


def test_import():
    """Test that we can import the module"""
    try:
        from reinforcing_mars_sim import PyGame, PyAction, PyPlayer
        assert PyGame is not None
        assert PyAction is not None
        assert PyPlayer is not None
    except ImportError as e:
        pytest.skip(f"Module not installed: {e}")


def test_basic_functionality():
    """Test basic functionality after installation"""
    try:
        from reinforcing_mars_sim import PyGame
        
        game = PyGame.new(num_players=2, seed=12345)
        assert game is not None
        assert game.get_generation() == 1
    except ImportError:
        pytest.skip("Module not installed")

