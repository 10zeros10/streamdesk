import { Component, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { StreamService } from './stream.service';

@Component({
  selector: 'app-streamdesk-dashboard',
  template: `
    <div class="stream-controls">
      <button (click)="activateStream()">Start Stream</button>
      <button (click)="deactivateStream()">Stop Stream</button>
      <button (click)="toggleViewerInteractionOverlay()">Toggle Live Interaction Overlay</button>
    </div>
    <div class="viewer-analytics">
      <p>Current Viewers: {{ currentViewerCount$ | async }}</p>
    </div>
  `,
  styleUrls: ['./streamdesk-dashboard.component.css']
})

export class StreamdeskDashboardComponent implements OnInit {
  
  currentViewerCount$: Observable<number>;

  constructor(private streamService: StreamService) { }

  ngOnInit(): void {
    this.fetchViewerCount();
  }

  private fetchViewerCount(): void {
    this.currentViewerCount$ = this.streamService.fetchCurrentViewerCount();
  }

  activateStream(): void {
    this.streamService.activateStreamService().subscribe({
      next: (response) => console.log('Stream activation successful', response),
      error: (error) => console.error('Error activating stream', error),
    });
  }

  deactivateStream(): void {
    this.streamService.deactivateStreamService().subscribe({
      next: (response) => console.log('Stream deactivation successful', response),
      error: (error) => console.error('Error deactivating stream', error),
    });
  }

  toggleViewerInteractionOverlay(): void {
    this.streamService.toggleViewerInteraction().subscribe({
      next: (response) => console.log('Viewer interaction overlay toggled', response),
      error: (error) => console.error('Error toggling viewer interaction overlay', error),
    });
  }
}