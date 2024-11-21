# Lotka-Volterra Predator-Prey Model Simulation

This project simulates the Lotka-Volterra predator-prey model using Rust and visualizes the results with Plotly.

## Description

The Lotka-Volterra equations describe the dynamics of biological systems in which two species interact, one as a predator and the other as prey. This project allows you to simulate and visualize these dynamics by adjusting parameters and initial conditions.

## Parameters

- **Prey Growth Rate (α)**: The rate at which the prey population grows.
- **Predator Death Rate (γ)**: The rate at which the predator population decreases in the absence of prey.
- **Predator Growth Rate per Prey Consumed (δ)**: The rate at which the predator population grows based on the number of prey consumed.
- **Prey Death Rate due to Predation (β)**: The rate at which the prey population decreases due to predation.
- **Initial Prey Population**: The starting number of prey.
- **Initial Predator Population**: The starting number of predators.
- **Time Period**: The total duration of the simulation.
- **Time Step**: The interval between each step of the simulation.

## How to Run

1. Install the required dependencies by adding `plotly` to your `Cargo.toml`:
    ```toml
    [dependencies]
    plotly = "0.10.0"
    ```

2. Compile and run the project using `cargo`:
    ```sh
    cargo run
    ```

3. Follow the prompts to input the parameters and initial conditions.

## Example

Here is an example of how to input the parameters:

Prey Growth Rate (α) (Default: 0.1): Predator Death Rate (γ) (Default: 0.1): Predator Growth Rate per Prey Consumed (δ) (Default: 0.01): Prey Death Rate due to Predation (β) (Default: 0.01): Initial Prey Population (thousands) (Default: 50.0): Initial Predator Population (thousands) (Default: 10.0): Time Period (weeks) (Default: 200.0): Time Step (weeks) (Default: 0.1): Prey name (Default: Rabbits): Predator name (Default: Wolves):


## Output

The simulation results will be displayed in a Plotly graph showing the population dynamics of the prey and predator over time.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes or improvements.

