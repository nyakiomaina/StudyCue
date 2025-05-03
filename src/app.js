import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

// DOM Elements
const weeklyPlanningSection = document.getElementById('weekly-planning');
const topicForm = document.getElementById('topic-form');
const topicsList = document.getElementById('topics-list');

// Check if it's Sunday and show weekly planning
const today = new Date();
if (today.getDay() === 0) { // 0 is Sunday
    weeklyPlanningSection.classList.remove('hidden');
}

// Listen for weekly planning event from backend
listen('weekly-planning', () => {
    weeklyPlanningSection.classList.remove('hidden');
});

// Handle topic form submission
topicForm.addEventListener('submit', async (e) => {
    e.preventDefault();

    const formData = {
        title: document.getElementById('title').value,
        description: document.getElementById('description').value || null,
        time_minutes: parseInt(document.getElementById('time-minutes').value),
        pages: document.getElementById('pages').value ? parseInt(document.getElementById('pages').value) : null,
        priority: document.getElementById('priority').value,
        estimated_time: parseInt(document.getElementById('estimated-time').value)
    };

    try {
        await invoke('add_topic', formData);
        topicForm.reset();
        loadTodayTopics();
    } catch (error) {
        console.error('Error adding topic:', error);
        alert('Failed to add topic: ' + error);
    }
});

// Load today's topics
async function loadTodayTopics() {
    try {
        const topics = await invoke('get_today_topics');
        displayTopics(topics);
    } catch (error) {
        console.error('Error loading topics:', error);
    }
}

// Display topics in the UI
function displayTopics(topics) {
    topicsList.innerHTML = '';

    if (topics.length === 0) {
        topicsList.innerHTML = '<p>No topics scheduled for today.</p>';
        return;
    }

    topics.forEach(topic => {
        const topicCard = document.createElement('div');
        topicCard.className = `topic-card priority-${topic.priority.toLowerCase()}`;

        topicCard.innerHTML = `
            <h3>${topic.title}</h3>
            ${topic.description ? `<p>${topic.description}</p>` : ''}
            <div class="topic-meta">
                <span>Time: ${topic.estimated_time} min</span>
                <span>Priority: ${topic.priority}</span>
            </div>
        `;

        topicsList.appendChild(topicCard);
    });
}

// Load initial topics
loadTodayTopics();

// Request daily notification
invoke('send_daily_notification').catch(error => {
    console.error('Error sending notification:', error);
});