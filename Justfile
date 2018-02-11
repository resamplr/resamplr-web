setup:
	@echo "Installing necessary stuff"
	npm install -g elm gulp --no-bin-links

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
	gulp build

# Run a development server and watch for changes
dev-run:
	@echo "Starting up the server"
	# TODO: gulp watch without blocking cargo run
	cargo run
	# TODO: Watch for changes

# ----------------- PROD ------------------ #

prod-build: prod-build-server prod-build-frontend

prod-build-server:
	@echo "Building server code"
	cargo build --release

prod-build-frontend:
	@echo "Building fronend code"
	gulp build

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

