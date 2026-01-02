"""Tests for Action types"""
import pytest
from reinforcing_mars_sim import PyAction, PyPayment, PyPaymentMethod, PyPaymentReserve


def test_pass_action():
    """Test creating a Pass action"""
    action = PyAction("Pass")
    assert action.action_type == "Pass"
    assert action.card_id is None
    assert action.payment is None


def test_convert_plants_action():
    """Test creating a ConvertPlants action"""
    action = PyAction("ConvertPlants")
    assert action.action_type == "ConvertPlants"


def test_convert_heat_action():
    """Test creating a ConvertHeat action"""
    action = PyAction("ConvertHeat")
    assert action.action_type == "ConvertHeat"


def test_play_card_action():
    """Test creating a PlayCard action"""
    action = PyAction("PlayCard")
    action.card_id = "test_card_1"
    
    # Create payment
    payment = PyPayment()
    payment.methods = [PyPaymentMethod("MegaCredits", 10)]
    action.payment = payment
    
    assert action.action_type == "PlayCard"
    assert action.card_id == "test_card_1"
    assert action.payment is not None
    assert len(action.payment.methods) == 1
    assert action.payment.methods[0].method_type == "MegaCredits"
    assert action.payment.methods[0].amount == 10


def test_standard_project_action():
    """Test creating a StandardProject action"""
    action = PyAction("StandardProject")
    action.project_type = "Greenery"
    
    # Create payment
    payment = PyPayment()
    payment.methods = [PyPaymentMethod("MegaCredits", 23)]
    action.payment = payment
    
    assert action.action_type == "StandardProject"
    assert action.project_type == "Greenery"
    assert action.payment is not None


def test_payment_methods():
    """Test different payment methods"""
    methods = [
        PyPaymentMethod("MegaCredits", 10),
        PyPaymentMethod("Steel", 5),
        PyPaymentMethod("Titanium", 3),
        PyPaymentMethod("Heat", 8),
        PyPaymentMethod("Plants", 4),
    ]
    
    for method in methods:
        assert method.amount > 0
        assert method.method_type in ["MegaCredits", "Steel", "Titanium", "Heat", "Plants"]


def test_payment_reserve():
    """Test PaymentReserve"""
    reserve = PyPaymentReserve()
    reserve.megacredits = 10
    reserve.steel = 5
    reserve.titanium = 3
    reserve.heat = 8
    reserve.plants = 4
    
    assert reserve.megacredits == 10
    assert reserve.steel == 5
    assert reserve.titanium == 3
    assert reserve.heat == 8
    assert reserve.plants == 4


def test_complex_payment():
    """Test payment with multiple methods"""
    payment = PyPayment()
    payment.methods = [
        PyPaymentMethod("MegaCredits", 5),
        PyPaymentMethod("Steel", 2),
    ]
    payment.reserve.megacredits = 3
    
    assert len(payment.methods) == 2
    assert payment.reserve.megacredits == 3

