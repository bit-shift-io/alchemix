# Alchemix

Alchemix is a chemistry-based spell mixing game. Spells are created by combining elements from the periodic table to form molecules. The properties of these molecules (determined by their atomic structure and bonding) define the effects of the resulting spells.

## Core Mechanics

- **Atomic Model**: Elements are defined by their protons, neutrons, and electrons.
- **Bonding**: Discrete electron shells determine the valency and compatibility of elements for covalent or ionic bonding.
- **Spell Synthesis**: Molecules act as the source for spell properties (e.g., combustion, freezing, toxicity).

## For AI Agents

This project uses a data-driven approach for elements. 
- `src/chemistry/elements.rs`: Contains the periodic table data.
- `src/chemistry/bonding.rs`: Contains the logic for determining if two atoms can bond.
- `src/spells/mapping.rs`: Maps molecular structures to game effects.

### Getting Started

```bash
cargo run
```

The CLI will prompt you to enter elements to attempt a reaction.
