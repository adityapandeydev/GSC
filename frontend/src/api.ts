export const getInvestmentAdvice = async (investmentType: string, riskLevel: string) => {
    const response = await fetch("http://localhost:8000/investment-advice", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ investment_type: investmentType, risk_level: riskLevel }),
    });

    return response.json();
};
