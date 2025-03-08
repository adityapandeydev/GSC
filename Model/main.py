# model/main.py
import json

def generate_investment_advice(user_input):
    """Simulate AI-based investment suggestions based on user input."""
    
    # Extract user preferences
    investment_type = user_input.get("investment_type", "general")
    risk_level = user_input.get("risk_level", "moderate")
    
    # Sample predefined advice
    advice = {
        "stocks": [
            "Diversify your stock portfolio across different sectors.",
            "Consider investing in ETFs for long-term stability.",
            "Keep an eye on market trends but avoid emotional trading."
        ],
        "real_estate": [
            "Look for properties in high-growth areas with good rental yield.",
            "Consider REITs if you want exposure without direct property ownership.",
            "Factor in property taxes, maintenance costs, and market trends."
        ],
        "gold_silver": [
            "Gold is a good hedge against inflation in economic uncertainty.",
            "Silver has industrial demand and can be more volatile than gold.",
            "Consider a mix of physical and paper assets (ETFs, mutual funds)."
        ],
        "general": [
            "Set a clear investment goal before putting in money.",
            "Diversification is key to reducing risks in any investment strategy.",
            "Always have an emergency fund before making high-risk investments."
        ]
    }
    
    # Return predefined suggestions based on investment type
    return advice.get(investment_type, advice["general"])

if __name__ == "__main__":
    # Simulate user input
    sample_user = {
        "investment_type": "stocks",
        "risk_level": "moderate"
    }
    
    response = generate_investment_advice(sample_user)
    print(json.dumps(response, indent=2))
