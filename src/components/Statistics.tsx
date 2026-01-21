import { Component, createSignal, onMount } from "solid-js";
import { getStatistics, Statistics as StatsType } from "../lib/tauri-api";
import "./Statistics.css";

const Statistics: Component = () => {
  const [stats, setStats] = createSignal<StatsType | null>(null);
  const [loading, setLoading] = createSignal(true);

  onMount(async () => {
    try {
      const data = await getStatistics();
      setStats(data);
    } catch (error) {
      console.error("Failed to load statistics:", error);
    } finally {
      setLoading(false);
    }
  });

  if (loading()) {
    return <div class="stats-container loading">Loading statistics...</div>;
  }

  const data = stats();

  return (
    <div class="stats-container">
      <header class="stats-header">
        <h1>Dashboard Statistics</h1>
        <p>Overview of your activity in Arce Cowork</p>
      </header>

      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-icon tasks">📋</div>
          <div class="stat-content">
            <h3>Total Tasks</h3>
            <div class="stat-value">{data?.total_tasks || 0}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon completed">✅</div>
          <div class="stat-content">
            <h3>Completed</h3>
            <div class="stat-value">{data?.completed_tasks || 0}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon messages">💬</div>
          <div class="stat-content">
            <h3>Conversations</h3>
            <div class="stat-value">{data?.total_conversations || 0}</div>
          </div>
        </div>

        <div class="stat-card">
          <div class="stat-icon activity">⚡</div>
          <div class="stat-content">
            <h3>Total Messages</h3>
            <div class="stat-value">{data?.total_messages || 0}</div>
          </div>
        </div>
      </div>

      <div class="stats-charts">
         <div class="chart-card">
             <h3>Task Success Rate</h3>
             <div class="chart-content">
                 <div class="progress-bar">
                     <div
                        class="progress-fill"
                        style={{
                            width: `${data?.total_tasks ? ((data.completed_tasks / data.total_tasks) * 100) : 0}%`
                        }}
                     ></div>
                 </div>
                 <p>{data?.total_tasks ? Math.round((data.completed_tasks / data.total_tasks) * 100) : 0}% Success Rate</p>
             </div>
         </div>
      </div>
    </div>
  );
};

export default Statistics;
