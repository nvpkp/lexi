# 🎯 Examples

Real-world Lexi programs to inspire your own projects.

## Basic Functions

### Fibonacci Sequence
```
# fibonacci.lxi
Create a function that generates the first n numbers in the Fibonacci sequence
Return an array starting with 0 and 1
Handle edge cases where n is 0 or 1
```

### Palindrome Checker
```
# palindrome.lxi  
Create a function that checks if a string is a palindrome
Ignore case, spaces, and punctuation
Return true for palindromes, false otherwise
```

## Web Development

### REST API Server
```
# api-server.lxi
Build a REST API server on port 3000
Create routes for GET /users, POST /users, DELETE /users/:id
Store users in memory with id, name, email properties
Add JSON parsing and CORS middleware
Return appropriate HTTP status codes for errors
```

### File Upload Handler
```
# upload.lxi
Create a web server that handles file uploads
Save files to an uploads directory with unique names
Validate file types and size limits
Return upload progress and completion status
```

## Data Processing

### CSV Analytics
```
# csv-analytics.lxi
Create a function that reads CSV files and calculates statistics
Compute mean, median, mode for numeric columns
Handle missing values with configurable defaults
Export results to JSON with proper formatting
```

### Log Parser
```
# log-parser.lxi
Build a web server log analyzer
Parse access logs to extract IPs, timestamps, and status codes
Count requests by status code and generate hourly statistics
Filter by date ranges and export to CSV
```

## Browser Testing

### Login Test Suite
```
# login-tests.lxi
Create automated tests for user login functionality
Open browser, navigate to login page, fill credentials
Verify successful login and dashboard redirect
Test invalid credentials and error message display
Take screenshots on failures and generate HTML reports
```

### E-commerce Testing
```
# shop-tests.lxi
Build tests for online shopping workflow
Add products to cart, update quantities, proceed to checkout
Test form validation and payment processing
Verify order confirmation emails
Test responsive design across viewports
```

## Algorithms

### Sorting Collection
```
# sorting.lxi
Implement quicksort with in-place sorting
Create merge sort for arrays of any comparable type  
Add function to sort objects by specified property
Include performance timing and comparison counting
```

### Graph Algorithms
```
# graph.lxi
Create graph data structure using adjacency lists
Implement breadth-first search for shortest paths
Add depth-first search for complete traversal
Create cycle detection for directed graphs
```

## System Tools

### File Organizer
```
# organizer.lxi
Build a file organizer that sorts by file type
Create folders for images, documents, videos, archives
Handle duplicate filenames with incremental numbering
Add dry-run mode to preview changes
Log all operations with timestamps
```

### Backup Script
```
# backup.lxi
Create a backup system for specified directories
Compress files and add timestamps to archive names
Support incremental backups based on modification dates
Send email notifications on completion or errors
```

## Machine Learning

### Linear Regression
```
# regression.lxi
Implement simple linear regression from scratch
Calculate slope, intercept, and R-squared values
Add prediction function for new data points
Include data visualization with scatter plots
Handle edge cases like vertical lines
```

### K-Means Clustering  
```
# kmeans.lxi
Implement K-means clustering algorithm
Initialize centroids with K-means++ method
Iterate until convergence with configurable tolerance
Calculate within-cluster sum of squares
Visualize clusters with different colors
```

## Game Development

### Text Adventure
```
# adventure.lxi
Create a text-based adventure game
Implement rooms, items, and player inventory
Add combat system with health and experience
Save and load game state to files
Include help system and command parsing
```

### Puzzle Solver
```
# sudoku.lxi
Build a Sudoku puzzle solver
Implement backtracking algorithm with constraint checking
Add puzzle validation and difficulty rating
Generate new puzzles with unique solutions
Create text-based interface for playing
```

## API Integrations

### Weather Dashboard
```
# weather.lxi
Create a weather dashboard using public APIs
Fetch current conditions and 5-day forecasts
Display temperature trends and precipitation charts
Cache responses to avoid rate limits
Add location search with autocomplete
```

### Social Media Scheduler
```
# scheduler.lxi
Build a social media post scheduler
Support multiple platforms with OAuth authentication
Schedule posts with images and hashtag suggestions
Track engagement metrics and analytics
Send notifications for scheduled posts
```
