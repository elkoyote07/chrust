# Chrust
The aim of this software is to gather and manage from a single place the chrontab.

### Initial research:
🔍 Start by researching how crontab files work and the details of their syntax. Familiarise yourself with the concepts of cron and how periodic tasks are scheduled on Unix systems.

### Design the data model:
👗 Define how you will represent the scheduled tasks in your application. You can create data structures to store the relevant information of each task, like the command to execute, the frequency, etc.

### Analysis and interpretation of the crontab syntax:
📚 Implement a parser that can read an existing crontab file or interpret user input and convert it into objects in your data model.

### Task scheduling:
📅 Create a scheduling system that periodically checks the scheduled tasks and executes those that correspond to the current moment.

### User interface:
👨‍💻 Design a command line interface (CLI) or a simple graphical user interface that allows users to add, delete and list scheduled tasks. You can use Rust libraries to facilitate the creation of user interfaces.

### Data persistence:
🎈 Decide how and where you will store scheduled task information. You can choose to store them in local files or use a database.

### Error handling and security:
💥 Make sure your application properly handles errors and avoids potential security vulnerabilities.

### Testing and validation:
🚀 Implement unit and integration tests to ensure that your crontab manager works correctly in different situations.