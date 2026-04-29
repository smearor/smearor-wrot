# Development Roadmap

## Overview

This roadmap outlines the development plan for smearor-wrot, a Wayland window rotation system for multi-user collaborative smart desks. The project is structured as a modular workspace with multiple crates, each serving a specific purpose in the rotation pipeline.

## Architecture

```
Wayland Application → smearor-wrot-core → smearor-wrot-gtk → smearor-wrot-rotation → Final Rotated Window
(Compositor)          (GTK4 Widget)       (Transformation)     (With Transformed Input)
```

## Development Phases

### Phase 1: Foundation (Current Status)

**Goal**: Establish basic project structure and dependencies

- [x] Workspace setup with Cargo
- [x] Dependency configuration (smithay, gtk4, clap, etc.)
- [x] Basic crate structure created
- [ ] Core error handling with `thiserror` and `miette`
- [ ] Basic logging infrastructure with `tracing`
- [ ] CI/CD pipeline setup
- [ ] Documentation structure

### Phase 2: Core Compositor (smearor-wrot-core)

**Goal**: Implement basic Wayland compositor functionality

**Priority**: High (foundation for all other components)

**Tasks**:
- [ ] Implement basic Wayland display server and set up Smithay compositor backend based on smallvil
- [ ] Implement surface management
- [ ] Add basic window lifecycle handling
- [ ] Implement buffer management
- [ ] Add basic rendering pipeline
- [ ] Error handling and recovery
- [ ] Unit tests for core functionality

**Estimated Duration**: 4-6 weeks

**Dependencies**: None (foundation layer)

### Phase 3: GTK4 Integration (smearor-wrot-gtk)

**Goal**: Create GTK4 widget for compositor rendering

**Priority**: High (required for rotation widget)

**Tasks**:
- [ ] Implement basic GTK4 widget
- [ ] Integrate with smearor-wrot-core
- [ ] Set up rendering surface
- [ ] Implement event handling
- [ ] Add window decoration support
- [ ] Implement resize and move operations
- [ ] Add keyboard/mouse input forwarding
- [ ] Integration tests with core

**Estimated Duration**: 3-4 weeks

**Dependencies**: smearor-wrot-core

### Phase 4: Rotation Transformation (smearor-wrot-rotation)

**Goal**: Implement input/output coordinate transformation

**Priority**: High (core feature)

**Tasks**:
- [ ] Implement rotation matrix mathematics
- [ ] Add output transformation (visual rotation)
- [ ] Implement input coordinate transformation
- [ ] Support arbitrary rotation angles (0-360°)
- [ ] Handle touch input transformation
- [ ] Add gesture support (pinch, rotate)
- [ ] Performance optimization (GPU acceleration)
- [ ] Comprehensive testing of transformations

**Estimated Duration**: 4-5 weeks

**Dependencies**: smearor-wrot-gtk

### Phase 5: CLI Wrapper (smearor-wrot-wrapper)

**Goal**: Create user-friendly command-line interface

**Priority**: Medium (usability)

**Tasks**:
- [ ] Implement CLI argument parsing with clap
- [ ] Add application launching logic
- [ ] Implement window size configuration
- [ ] Add fullscreen/maximized modes
- [ ] Support custom Wayland sockets
- [ ] Add configuration file support
- [ ] Implement daemon mode
- [ ] User documentation and examples

**Estimated Duration**: 2-3 weeks

**Dependencies**: All other crates

### Phase 6: Test Client (smearor-wrot-test-client)

**Goal**: Create test application for development

**Priority**: Medium (development tool)

**Tasks**:
- [ ] Implement basic test application
- [ ] Add visual rotation testing
- [ ] Test input transformation
- [ ] Performance benchmarking
- [ ] Multi-window testing
- [ ] Automated test suite

**Estimated Duration**: 2 weeks

**Dependencies**: All other crates

### Phase 7: Advanced Features

**Goal**: Add advanced functionality for production use

**Priority**: Medium (enhancement)

**Tasks**:
- [ ] DMA-BUF support for hardware acceleration
- [ ] Multi-window support with independent rotation
- [ ] Window persistence across sessions
- [ ] Configuration profiles
- [ ] Hot-reload configuration
- [ ] Performance profiling tools
- [ ] Memory optimization
- [ ] Security hardening

**Estimated Duration**: 6-8 weeks

**Dependencies**: All core features complete

### Phase 8: Cross-Desktop Compatibility

**Goal**: Ensure compatibility with major Wayland compositors

**Priority**: High (usability)

**Tasks**:
- [ ] Hyprland compatibility testing
- [ ] Sway compatibility testing
- [ ] GNOME compatibility testing
- [ ] KDE Plasma compatibility testing
- [ ] Wayfire compatibility testing
- [ ] Compositor-specific workarounds
- [ ] Automated compatibility testing

**Estimated Duration**: 3-4 weeks

**Dependencies**: All core features complete

### Phase 9: Documentation and Examples

**Goal**: Comprehensive documentation for users and developers

**Priority**: High (usability and adoption)

**Tasks**:
- [ ] User guide with examples
- [ ] API documentation
- [ ] Architecture documentation
- [ ] Contributing guidelines
- [ ] Troubleshooting guide
- [ ] Video tutorials
- [ ] Example configurations
- [ ] Integration examples

**Estimated Duration**: 2-3 weeks

**Dependencies**: All features complete

### Phase 10: Production Readiness

**Goal**: Prepare for stable release

**Priority**: High (release)

**Tasks**:
- [ ] Comprehensive testing suite
- [ ] Security audit
- [ ] Performance optimization
- [ ] Memory leak testing
- [ ] Long-running stability tests
- [ ] Package creation (deb, rpm, arch)
- [ ] Flatpak packaging
- [ ] Release notes and changelog
- [ ] Version 1.0.0 release

**Estimated Duration**: 4-6 weeks

**Dependencies**: All previous phases complete

## Milestones

### Milestone 1: MVP (Minimum Viable Product)
- Complete Phases 1-5
- Basic window rotation working
- CLI interface functional
- Target: 3-4 months

### Milestone 2: Beta Release
- Complete Phases 1-7
- Advanced features implemented
- Multi-window support
- Target: 5-6 months

### Milestone 3: Production Release
- Complete Phases 1-10
- Cross-desktop compatibility
- Comprehensive documentation
- Target: 8-10 months

## Risk Assessment

### High Risk Items
1. **Smithay Integration**: Smithay is complex and may require deep Wayland knowledge
   - Mitigation: Start with simple compositor, incrementally add features
2. **Input Transformation**: Coordinate transformation accuracy critical
   - Mitigation: Extensive testing with real touch devices
3. **Performance**: 60 FPS target may be challenging
   - Mitigation: Early profiling, GPU acceleration from start

### Medium Risk Items
1. **GTK4 Integration**: GTK4 API changes and stability
   - Mitigation: Pin versions, monitor upstream changes
2. **Cross-Desktop Compatibility**: Different compositor behaviors
   - Mitigation: Early testing on multiple compositors
3. **Hardware Acceleration**: DMA-BUF support complexity
   - Mitigation: Fallback to software rendering

### Low Risk Items
1. **CLI Interface**: Well-understood problem space
2. **Documentation**: Can be done incrementally
3. **Testing**: Standard practices apply

## Resource Requirements

### Development Resources
- Senior Rust developer with Wayland experience
- GTK4 development experience
- Graphics programming knowledge
- Linux system programming expertise

### Hardware Requirements
- Development machine with Wayland compositor
- Touch-enabled display for testing
- Multiple Wayland compositors for compatibility testing
- GPU with DMA-BUF support

### Time Requirements
- Full-time development: 8-10 months to v1.0
- Part-time development: 18-24 months to v1.0

## Dependencies

### External Dependencies
- Rust 1.87+
- GTK4 development libraries
- Wayland development libraries
- Smithay framework
- Linux with Wayland compositor

### Internal Dependencies (Crates)
- smearor-wrot-core: No dependencies
- smearor-wrot-gtk: Depends on smearor-wrot-core
- smearor-wrot-rotation: Depends on smearor-wrot-gtk
- smearor-wrot-wrapper: Depends on all other crates
- smearor-wrot-test-client: Depends on all other crates

## Testing Strategy

### Unit Testing
- Each crate has comprehensive unit tests
- Test coverage target: 80%+
- Continuous integration running on every commit

### Integration Testing
- Test interactions between crates
- End-to-end testing with real applications
- Automated testing on multiple compositors

### Performance Testing
- Benchmark rendering performance (target: 60 FPS)
- Memory usage monitoring
- Input latency measurement

### User Acceptance Testing
- Beta testing with real smart desk setups
- User feedback collection
- Usability testing

## Success Criteria

### Technical Success
- [ ] Maintains 60 FPS with typical applications
- [ ] Input transformation accuracy < 1 pixel error
- [ ] Memory usage < 200 MB per rotated window
- [ ] Compatible with major Wayland compositors
- [ ] Zero security vulnerabilities in audit

### User Success
- [ ] Easy to install and configure
- [ ] Intuitive CLI interface
- [ ] Comprehensive documentation
- [ ] Active community support
- [ ] Positive user feedback

## Next Steps

1. **Immediate**: Complete Phase 1 foundation tasks
2. **Short-term**: Begin Phase 2 core compositor development
3. **Medium-term**: Implement GTK4 integration and rotation
4. **Long-term**: Advanced features and production readiness

## Notes

- This roadmap is a living document and will be updated as development progresses
- Priorities may shift based on user feedback and technical challenges
- Estimated durations are approximate and may vary
- Community contributions are welcome and encouraged
