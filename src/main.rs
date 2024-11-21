use std::io;
use plotly::color::NamedColor;
use plotly::common::{Line, Mode};
use plotly::{Plot, Scatter};

fn main() {
    let prey_growth_rate = get_input("Prey growth rate (α): ", 0.1);
    let pred_death_rate = get_input("Predator death rate (γ): ", 0.1);
    let pred_growth_rate = get_input("Predator growth rate per prey consumed (δ): ", 0.01);
    let prey_death_rate = get_input("Prey death rate due to predation (β): ", 0.01);
    let initial_prey = get_input("Initial prey population (thousands): ", 50.0);
    let initial_pred = get_input("Initial predator population (thousands): ", 10.0);
    let time_period = get_input("Time period (weeks): ", 200.0) as usize;
    let time_step = get_input("Time step (weeks): ", 0.1);

    let prey_name = get_string_input("Prey name: ", "Rabbits".to_string());
    let pred_name = get_string_input("Predator name: ", "Wolves".to_string());

    // Run simulation
    let (times, prey_counts, pred_counts) = simulate_lotka_volterra(
        prey_growth_rate,
        pred_death_rate,
        pred_growth_rate,
        prey_death_rate,
        initial_prey,
        initial_pred,
        time_period,
        time_step,
    );

    // Plot results
    plot(prey_name, pred_name, times, prey_counts, pred_counts);
}

fn get_input(prompt: &str, default: f64) -> f64 {
    println!("{prompt}(Default: {default}):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim().is_empty() {
        default
    } else {
        input.trim().parse().unwrap_or(default)
    }
}

fn get_string_input(prompt: &str, default: String) -> String {
    println!("{prompt}(Default: {default}):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if input.trim().is_empty() {
        default
    } else {
        input.trim().to_string()
    }
}

fn simulate_lotka_volterra(
    prey_growth_rate: f64,  // α: Prey reproduction rate
    pred_death_rate: f64,   // γ: Predator death rate
    pred_growth_rate: f64,  // δ: Predator reproduction rate per prey consumed
    prey_death_rate: f64,   // β: Prey death rate due to predation
    initial_prey: f64,      // Initial prey population
    initial_pred: f64,      // Initial predator population
    time_period: usize,     // Total simulation time
    time_step: f64,         // Simulation time step
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut times = Vec::new();
    let mut prey_counts = Vec::new();
    let mut pred_counts = Vec::new();

    let mut prey = initial_prey;
    let mut pred = initial_pred;

    for step in 0..(time_period as f64 / time_step) as usize {
        // Record time and populations
        times.push(step as f64 * time_step);
        prey_counts.push(prey);
        pred_counts.push(pred);

        // Calculate rate of change using Lotka–Volterra equations
        let prey_change = (prey_growth_rate * prey - prey_death_rate * prey * pred) * time_step;
        let pred_change = (pred_growth_rate * prey * pred - pred_death_rate * pred) * time_step;

        // Update populations
        prey += prey_change;
        pred += pred_change;

        // Ensure populations don't go negative
        if prey < 0.0 {
            prey = 0.0;
        }
        if pred < 0.0 {
            pred = 0.0;
        }
    }

    (times, prey_counts, pred_counts)
}

fn plot(prey_name: String, pred_name: String, times: Vec<f64>, prey_counts: Vec<f64>, pred_counts: Vec<f64>) {
    let prey_trace = Scatter::new(times.clone(), prey_counts)
        .mode(Mode::LinesMarkers)
        .name(prey_name)
        .line(Line::new().color(NamedColor::Gold).width(3.0));

    let pred_trace = Scatter::new(times, pred_counts)
        .mode(Mode::LinesMarkers)
        .name(pred_name)
        .line(Line::new().color(NamedColor::Crimson).width(3.0));

    let mut plot = Plot::new();
    plot.add_trace(prey_trace);
    plot.add_trace(pred_trace);

    plot.show();
}
