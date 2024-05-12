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
