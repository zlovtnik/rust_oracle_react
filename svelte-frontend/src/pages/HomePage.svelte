<script lang="ts">
    import { Tile, Grid, Row, Column, Button } from "carbon-components-svelte";
    import {
        Launch,
        Document,
        Application,
        Collaborate,
    } from "carbon-icons-svelte";

    // Accept navigateTo function as a prop
    export let navigateTo: (path: string) => void;
</script>

<div class="welcome-page">
    <div class="hero-section">
        <div class="hero-content">
            <h1>Welcome to NFe Management System</h1>
            <div class="accent-bar"></div>
            <p class="hero-description">
                Manage your electronic invoices with our intuitive application.
                Simplify your tax documentation process and stay compliant.
            </p>
        </div>
        <div class="hero-buttons">
            <Button icon={Launch} on:click={() => navigateTo("/ide")}>
                Get Started
            </Button>
            <a
                href="https://www.gov.br/receitafederal/pt-br/assuntos/orientacao-tributaria/notas-fiscais/nfe"
                target="_blank"
                rel="noopener noreferrer"
            >
                <Button kind="tertiary">Learn About NFe</Button>
            </a>
        </div>
        <div class="hero-decoration">
            <div class="decoration-circle circle-1"></div>
            <div class="decoration-circle circle-2"></div>
            <div class="decoration-circle circle-3"></div>
        </div>
    </div>

    <div class="features-section">
        <h2>Key Features</h2>
        <div class="accent-bar centered"></div>

        <Grid>
            <Row>
                <Column sm={4} md={4} lg={4}>
                    <Tile class="feature-tile">
                        <div class="feature-icon">
                            <Document size={32} />
                        </div>
                        <h3>Document Management</h3>
                        <p>
                            Efficiently organize and track all your electronic
                            invoices in one place.
                        </p>
                    </Tile>
                </Column>

                <Column sm={4} md={4} lg={4}>
                    <Tile class="feature-tile">
                        <div class="feature-icon">
                            <Application size={32} />
                        </div>
                        <h3>Easy Integration</h3>
                        <p>
                            Seamlessly integrate with your existing systems and
                            business workflows.
                        </p>
                    </Tile>
                </Column>

                <Column sm={4} md={4} lg={4}>
                    <Tile class="feature-tile">
                        <div class="feature-icon">
                            <Collaborate size={32} />
                        </div>
                        <h3>Team Collaboration</h3>
                        <p>
                            Work together with your team on invoice management
                            and reporting.
                        </p>
                    </Tile>
                </Column>
            </Row>
        </Grid>
    </div>

    <div class="stats-section">
        <div class="stat-item">
            <div class="stat-number">100%</div>
            <div class="stat-label">Compliance</div>
        </div>
        <div class="stat-item">
            <div class="stat-number">50%</div>
            <div class="stat-label">Time Saved</div>
        </div>
        <div class="stat-item">
            <div class="stat-number">24/7</div>
            <div class="stat-label">Availability</div>
        </div>
    </div>
</div>
