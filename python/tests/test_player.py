"""Tests for Player wrapper"""
import pytest
from reinforcing_mars_sim import PyGame


def test_player_attributes():
    """Test player attributes"""
    game = PyGame.new(num_players=2, seed=12345)
    player = game.get_player("Player 1")
    
    assert player.id == "Player 1"
    assert player.name == "Player 1"
    assert isinstance(player.terraform_rating, int)
    assert isinstance(player.victory_points, int)
    assert player.terraform_rating >= 0
    assert player.victory_points >= 0


def test_player_resources():
    """Test getting player resources"""
    game = PyGame.new(num_players=2, seed=12345)
    player = game.get_player("Player 1")
    
    resources = player.get_resources()
    assert resources is not None
    # Resources should be a dict (even if empty for now)


def test_player_production():
    """Test getting player production"""
    game = PyGame.new(num_players=2, seed=12345)
    player = game.get_player("Player 1")
    
    production = player.get_production()
    assert production is not None
    # Production should be a dict (even if empty for now)


def test_player_cards():
    """Test getting player cards"""
    game = PyGame.new(num_players=2, seed=12345)
    player = game.get_player("Player 1")
    
    cards_in_hand = player.get_cards_in_hand()
    played_cards = player.get_played_cards()
    
    assert isinstance(cards_in_hand, list)
    assert isinstance(played_cards, list)


def test_multiple_players():
    """Test multiple players"""
    game = PyGame.new(num_players=4, seed=12345)
    players = game.get_players()
    
    assert len(players) == 4
    assert players[0].id == "Player 1"
    assert players[1].id == "Player 2"
    assert players[2].id == "Player 3"
    assert players[3].id == "Player 4"

