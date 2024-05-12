# gitlab-stats

- **Create an Action Secret**:
    - Navigate to the Settings of your GitHub repository.
    - Go to the Secrets and variables section.
    - Click on Actions and 'New repository secret'.
    - Name the secret `GITLAB_USERNAME` and provide the appropriate GitLab username as its value.

- **Add the Secret to your `README.md`**:
    - Embed the action secret in your GitHub Actions workflow by enclosing it within the workflow file. This does not expose the secret in your `README.md`, but you can indicate where the data will appear.
    - Edit your `README.md` to include the following section where the GitLab username will be utilized:

      ```markdown
      <!--START_SECTION:emo-gitlab-->
      <!--END_SECTION:emo-gitlab-->
      ```

      This section can be used by a GitHub Actions workflow to insert dynamically generated content, such as statistics or updates related to the GitLab username stored in your secret.

- **Add workflow to your profile repo `.github/workflows`**:
    ```markdown
  name: Gitlab Stats update
  on:
    workflow_dispatch:
    schedule:
        - cron: "0 0 * * *"

  jobs:
    update-readme:
        runs-on: ubuntu-latest


    steps:
      - name: Checkout ${{ github.event.repository.name }} repository
        uses: actions/checkout@v2
        with:
          path: ${{ github.event.repository.name }}

      - name: Checkout gitlab-stats repository
        uses: actions/checkout@v2
        with:
          repository: 'emochka2007/gitlab-stats'
          token: ${{ secrets.GITHUB_TOKEN }}   # Use a GitHub Secret for the token
          path: 'gitlab_stats'

      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Build and run Rust application
        run: cargo run ${{ secrets.GITLAB_USERNAME }} > ../${{ github.event.repository.name }}/output.md
        working-directory: gitlab_stats

      - name: Update README
        run: |
            sed -i '/^<!--START_SECTION:emo-gitlab-->$/,/^<!--END_SECTION:emo-gitlab-->/{//!d;}' README.md
            sed -i '/^<!--START_SECTION:emo-gitlab-->/r ./output.md' README.md
        working-directory: ${{ github.event.repository.name }}

      - name: Commit and push changes
        run: |
            git config --global user.name ${{ github.actor }}
            git config user.email "${{ github.actor_id }}+${{ github.actor }}@users.noreply.github.com"
            git add README.md
            git commit -m "Updated README with new stats"
            git push
        working-directory: ${{ github.event.repository.name }}
