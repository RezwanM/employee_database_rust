# employee_database_rust

This repo contains code for maintaining a database for all employees within a company. The user can add employees to a department, fetch the list of employees belonging to a particular department, and print out the list of all employees of the company according to their departments, in alphabetical order.  

The project was written in Rust.

## Preview

    $ cd <project_root> 
    $ cargo run
        Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
         Running `target\debug\employee_database.exe`
    Welcome to the Company Employee Database!
    Select an option:
    (1) Add an employee
    (2) Get employee names within a department
    (3) Get all employee names by department
    (4) Quit
    1
    Enter a new employee info in the format 'Add <employee_name> to <department_name>':
    e.g., Add Sally to Engineering
    Add Rezwan to Engineering


    Select an option:
    (1) Add an employee
    (2) Get employee names within a department
    (3) Get all employee names by department
    (4) Quit
    2
    Enter department name:
    Engineering
    Members of the department are:
    ["Rezwan"]

    Select an option:
    (1) Add an employee
    (2) Get employee names within a department
    (3) Get all employee names by department
    (4) Quit
    3
    Here are the employees by department:
    Engineering:
    ["Rezwan"]

    Select an option:
    (1) Add an employee
    (2) Get employee names within a department
    (3) Get all employee names by department
    (4) Quit
    4

## Requirements

- rustc 1.84.0 (9fc6b4312 2025-01-07)

## Setup
The following instructions assume the user has Ubuntu as their local machine's OS. Most instructions should work for other Linux distributions as well, though mileage may vary.

### Step 1: Install Rust
Set up Rust (v1.84.0) on the local machine.

    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    
### Step 2: Clone the project
Clone this GitHub repository into the local machine.
    
    git clone --single-branch -b main <project_repo_url> <project_root> 
    
### Step 3: Resolve dependencies 
Install missing libraries required for running the project (if any).
    
    cargo build
    
### Step 4: Run the application
Run the application from the command-line. For quitting the application, type "4" and press Enter.

## Usage
    
    cd <project_root>
    cargo run

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
