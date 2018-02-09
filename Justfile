setup:
	@echo "Installing necessary stuff"
	npm install

# ----------------- DEV ------------------ #

# Build rust server and frontend application
dev-build: dev-build-server dev-build-frontend
	@echo "Finished building everything"

# Build rust server only
dev-build-server:
	@echo "Building server code"
	cargo build

# Build elm frontend only
dev-build-frontend:
	@echo "Building fontend code"
	gulp

# Run a development server
dev-serve:
	@echo "Starting up the server"
	cargo run

# ----------------- PROD ------------------ #

prod-build: prod-build-server prod-build-frontend

prod-build-server:
	@echo "Building server code"
	cargo build --release

prod-build-frontend:
	@echo "Building fronend code"
	gulp

# ----------------- GENERAL ------------------ #

# Cleanup all builds
clean-all: clean-server clean
	@echo "Finished cleanup"

# Only clean server files
clean-server:
	@echo "Cleaning up server build"
	rm -rf target

# Only clean frontend stuff
clean:
	@echo "Cleaning up frontend build"
	rm -rf public

