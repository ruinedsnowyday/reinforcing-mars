"""Pytest configuration and fixtures"""
import pytest
from reinforcing_mars_sim import PyGame


@pytest.fixture
def game_2p():
    """Fixture for a 2-player game"""
    return PyGame.new(num_players=2, seed=12345)


@pytest.fixture
def game_4p():
    """Fixture for a 4-player game"""
    return PyGame.new(num_players=4, seed=54321)

