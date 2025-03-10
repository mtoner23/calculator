type Brackets = [(f64, f64); 6];

const _SINGLE_TAX_BRACKETS_2024: Brackets = [
    (11_600.0, 0.10),
    (47_150.0, 0.12),
    (100_525.0, 0.22),
    (191_950.0, 0.24),
    (243_725.0, 0.32),
    (609_350.0, 0.35),
];

const _JOINT_TAX_BRACKETS_2024: Brackets = [
    (23200.0, 0.10),
    (94300.0, 0.12),
    (201050.0, 0.22),
    (383900.0, 0.24),
    (487450.0, 0.32),
    (731200.0, 0.35),
];

const SINGLE_TAX_BRACKETS_2025: Brackets = [
    (11925.0, 0.10),
    (48475.0, 0.12),
    (103350.0, 0.22),
    (197300.0, 0.24),
    (250525.0, 0.32),
    (626350.0, 0.35),
];

const JOINT_TAX_BRACKETS_2025: Brackets = [
    (23850.0, 0.10),
    (96950.0, 0.12),
    (206700.0, 0.22),
    (394600.0, 0.24),
    (501050.0, 0.32),
    (751600.0, 0.35),
];

pub const BONUS_WITHHELD_RATE: f64 = 0.22;
pub const TOP_INCOME_TAX_RATE: f64 = 0.37;

pub fn calculate_income_tax(income: f64, status: FilingStatus) -> f64 {
    let brackets = get_brackets(status);
    let mut tax = 0.0;
    let mut previous_bracket = 0.0;

    for &(bracket, rate) in &brackets {
        if income > bracket {
            tax += (bracket - previous_bracket) * rate;
            previous_bracket = bracket;
        } else {
            tax += (income - previous_bracket) * rate;
            return tax;
        }
    }

    // Apply the top tax rate for income above the last bracket
    tax += (income - previous_bracket) * TOP_INCOME_TAX_RATE;
    tax
}

pub fn _get_marginal_tax_rate(income: f64, status: FilingStatus) -> f64 {
    let brackets = get_brackets(status);

    for &(bracket, rate) in &brackets {
        if income > bracket {
            continue;
        } else {
            return rate;
        }
    }
    return TOP_INCOME_TAX_RATE;
}

fn get_brackets(status: FilingStatus) -> Brackets {
    match status {
        FilingStatus::MarriedJoint => JOINT_TAX_BRACKETS_2025,
        FilingStatus::MarriedSeparate => SINGLE_TAX_BRACKETS_2025,
        FilingStatus::Single => SINGLE_TAX_BRACKETS_2025,
    }
}

pub fn get_standard_decution(status: FilingStatus) -> f64 {
    const SINGLE_DEDUCTION: f64 = 15000.0;
    const JOINT_DEDUCTION: f64 = 30000.0;

    match status {
        FilingStatus::Single => SINGLE_DEDUCTION,
        FilingStatus::MarriedSeparate => SINGLE_DEDUCTION,
        FilingStatus::MarriedJoint => JOINT_DEDUCTION,
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum FilingStatus {
    Single,
    MarriedJoint,
    MarriedSeparate,
}
