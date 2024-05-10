import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { StreamService } from './stream.service';

@Component({
  selector: 'app-streamdesk-dashboard',
  template: `
    <div class="stream-controls">
      <button (click)="startStream()">Start Stream</button>
      <button (click)="stopStream()">Stop Stream</button>
      <button (click)="toggleLiveInteractionOverlay()">Toggle Live Interaction Overlay</button>
    </div>
    <div class="viewer-analytics">
      <p>Current Viewers: {{ viewerCount$ | async }}</p>
    </div>
  `,
  styleUrls: ['./streamdesk-dashboard.component.css']
})

export class StreamdeskDashboardComponent implements OnInit {
  
  viewerCount$: Observable<number>;

  constructor(private streamService: StreamService) { }

  ngOnInit(): void {
    this.initializeViewerCount();
  }

  private initializeViewerCount(): void {
    this.viewerCount$ = this.streamService.getViewerCount();
  }

  startStream(): void {
    this.streamService.startStream().subscribe({
      next: (response) => console.log('Stream started', response),
      error: (error) => console.error('Error starting stream', error),
    });
  }

  stopStream(): void {
    this.streamService.stopStream().subscribe({
      next: (response) => console.log('Stream stopped', response),
      error: (error) => console.error('Error stopping stream', error),
    });
  }

  toggleLiveInteractionOverlay(): void {
    this.streamService.toggleLiveInteractionOverlay().subscribe({
      next: (response) => console.log('Overlay toggled', response),
      error: (error) => console.error('Error toggling overlay', error),
    });
  }
}