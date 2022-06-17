;; shadowplay.el --- Flycheck binding to shadowplay -*- lexical-binding: t; -*-

;; Copyright (C) 2021-2022 Evgenii Lepikhin
;; Copyright (C) 2021-2022 VK Company
;;
;; Author: Evgenii Lepikhin <johnlepikhin@gmail.com>
;; Maintainer: Evgenii Lepikhin <johnlepikhin@gmail.com>

;;; Commentary:

;;; Code:

(defcustom shadowplay-program "shadowplay"
  "*Program name of shadowplay"
  :type 'string
  :group 'shadowplay)

(defcustom shadowplay-repository-path "./"
  "*Path to the root of Puppet repository"
  :type 'string
  :group 'shadowplay)

(defun flycheck-parse-shadowplay-lint (output checker buffer)
  "Parse JSON OUTPUT of CHECKER on BUFFER as Shadowplay errors."
  (mapcar (lambda (err)
            (let-alist err
              (flycheck-error-new
               :line .range.start.line
               :column .range.start.column
               :level (pcase .error_type
                 ("ManifestSyntax" 'error)
                 ("ManifestLint" 'warning)
                 ("Hiera" 'warning)
                 ("YAML" 'warning)
                 (_ 'error))
               :message .message
               :end-line (and .range.end.line .range.end.line)
               :end-column (and .range.end.column (+ .range.end.column 1))
               :checker checker
               :buffer buffer
               :filename .range.path)))
          (flycheck-parse-json output)))

(flycheck-define-checker puppet-shadowplay-pp
  "A Puppet PP DSL linter using shadowplay."
  :command ("shadowplay"
            "--repo-path" (eval shadowplay-repository-path)
            "check"
            "-f"
            "json"
            "pp"
            source)
  :error-parser flycheck-parse-shadowplay-lint
  :modes puppet-mode)

(flycheck-define-checker puppet-shadowplay-hiera-yaml
  "A Puppet PP DSL linter using shadowplay."
  :command ("shadowplay"
            "--repo-path" (eval shadowplay-repository-path)
            "check"
            "-f"
            "json"
            "hiera"
            source)
  :error-parser flycheck-parse-shadowplay-lint
  :modes yaml-mode
  :enabled (lambda () (string-match-p "/hieradata" (buffer-file-name))))

(add-to-list 'flycheck-checkers 'puppet-shadowplay-pp)
(add-to-list 'flycheck-checkers 'puppet-shadowplay-hiera-yaml)


(defconst shadowplay-pretty-print "*shadowplay-pretty-print*")

;; based on rust--format-call from rust-rustfmt
(defun shadowplay-format-buffer ()
  "Format BUF using shadowplay."
  (interactive)
  (or (get 'shadowplay-program 'has-shadowplay)
      (if (executable-find shadowplay-program)
          (put 'shadowplay-program 'has-shadowplay t)
        (error "Seem shadowplay is not installed")))

  (let ((buf (current-buffer))
        (shadowplay-run-list '("pretty-print-pp")))
    (with-current-buffer (get-buffer-create shadowplay-pretty-print)
      (view-mode +1)
      (let ((inhibit-read-only t))
        (erase-buffer)
        (insert-buffer-substring buf)
        (let* ((tmpf (make-temp-file "shadowplay"))
               (ret (apply 'call-process-region
                           (point-min)
                           (point-max)
                           shadowplay-program
                           t
                           `(t ,tmpf)
                           nil
                           shadowplay-run-list)))
          (unwind-protect
              (cond
               ((zerop ret)
                (if (not (string= (buffer-string)
                                  (with-current-buffer buf (buffer-string))))
                    ;; replace-buffer-contents was in emacs 26.1, but it
                    ;; was broken for non-ASCII strings, so we need 26.2.
                    (if (and (fboundp 'replace-buffer-contents)
                             (version<= "26.2" emacs-version))
                        (with-current-buffer buf
                          (replace-buffer-contents shadowplay-pretty-print))
                      (copy-to-buffer buf (point-min) (point-max))))
                (kill-buffer))
               ((= ret 3)
                (if (not (string= (buffer-string)
                                  (with-current-buffer buf (buffer-string))))
                    (copy-to-buffer buf (point-min) (point-max)))
                (erase-buffer)
                (insert-file-contents tmpf)
                (error "Shadowplay could not format some lines, see %s buffer for details"
                       shadowplay-pretty-print))
               (t
                (erase-buffer)
                (insert-file-contents tmpf)
                (error "Shadowplay failed, see %s buffer for details"
                       shadowplay-pretty-print))))
          (delete-file tmpf))))))

(provide 'shadowplay)

;;; shadowplay.el ends here
