import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import CalendarEvents from "@/components/ui/calendar-events";
import SocialMediaHeader from "@/app/dashboard/modules/social-media/social-media-header";
import { EventKind } from "@/lib/types";

export interface SocialMediaPost {
  id?: number;
  event_id?: number;
  platform: string;
  content: string;
  status: string;
  schedule_time: string;
}

export default function SocialMediaMain() {
  const [posts, setPosts] = useState<SocialMediaPost[]>([]);

  useEffect(() => {
    fetchPosts();
  }, []);

  const fetchPosts = async () => {
    try {
      const response: SocialMediaPost[] = await invoke("list_social_posts");
      setPosts(response);
    } catch (error) {
      console.error("Error fetching posts:", error);
    }
  };

  return (
    <div>
      <SocialMediaHeader fetchPosts={fetchPosts} />
      <CalendarEvents eventKind={EventKind.SocialMedia} />
      
      <div className="p-5">
        <h2 className="text-lg font-semibold mb-3">Scheduled & Recent Posts</h2>
        {posts.length === 0 ? (
          <p>No posts yet.</p>
        ) : (
          posts.map((post) => (
            <div key={post.id} className="p-3 border rounded-lg shadow-md mb-3">
              <h3 className="font-bold">{post.platform}</h3>
              <p>{post.content}</p>
              <p className="text-sm text-muted">Scheduled: {post.schedule_time}</p>
              <p className={`text-xs ${post.status === "Posted" ? "text-green-600" : "text-yellow-600"}`}>
                Status: {post.status}
              </p>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
