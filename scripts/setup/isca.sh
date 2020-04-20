# Replace fjmw201 with your username!

# First, login to isca:
ssh fjmw201@login.isca.ex.ac.uk;

# We need the Rust and NetCDF modules. The GCC module conflicts with the NetCDF module we need.
module load Rust/1.41.0-GCCcore-8.2.0;
module unload GCCcore/8.2.0;
module load netCDF/4.6.1-foss-2018b;

# So executable can find the install location.
export DIA_DIR="/gpfs/ts0/home/fjmw201/Projects/dia";

# Create projects directory.
cd .;
mkdir Projects;
cd Projects;

# Clone dia repo and check that it will compile.
git clone https://github.com/FreddyWordingham/arc.git;
cd dia;
cargo check;

# Run the binary with the parameters input file.
cargo run --bin main parameters.json;
