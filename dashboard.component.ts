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
    <div class="stream-status">
      <p>Stream Status: {{ streamStatus }}</p>
      <p>Viewer Interaction Overlay: {{ viewerInteractionOverlay ? 'On' : 'Off' }}</p>
    </div>
    <div class="viewer-analytics">
      <p>Current Viewers: {{ currentViewerCount$ | async }}</p>
    </div>
  `,
  styleUrls: ['./streamdesk-dashboard.component.css']
})

export class StreamdeskDashboardComponent implements OnInit {
  
  currentViewerCount$: Observable<number>;
  streamStatus: string = 'Inactive'; // Initial stream status
  viewerInteractionOverlay: boolean = false; // Initial viewer interaction overlay state

  constructor(private streamService: StreamService) { }

  ngOnInit(): void {
    this.fetchViewerCount();
  }

  private fetchViewerCount(): void {
    this.currentViewerCount$ = this.streamService.fetchCurrentViewerCount();
  }

  activateStream(): void {
    this.streamService.activateStreamService().subscribe({
      next: (response) => {
        console.log('Stream activation successful', response);
        this.streamStatus = 'Active'; // Update stream status
      },
      error: (error) => {
        console.error('Error activating stream', error);
        this.streamStatus = 'Error activating stream'; // Reflect potential error in status
      },
    });
  }

  deactivateStream(): void {
    this.streamService.deactivateStreamService().subscribe({
      next: (response) => {
        console.log('Stream deactivation successful', response);
        this.streamStatus = 'Inactive'; // Update stream status
      },
      error: (error) => {
        console.error('Error deactivating stream', error);
        this.streamStatus = 'Error deactivating stream'; // Reflect potential error in status
      },
    });
  }

  toggleViewerInteractionOverlay(): void {
    this.streamService.toggleViewerInteraction().subscribe({
      next: (response) => {
        console.log('Viewer interaction overlay toggled', response);
        this.viewerInteractionOverlay = !this.viewerInteractionOverlay; // Toggle viewer interaction overlay state
      },
      error: (error) => {
        console.error('Error toggling viewer interaction overlay', error);
      },
    });
  }
}