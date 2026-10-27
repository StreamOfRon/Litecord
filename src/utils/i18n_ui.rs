//! All UI strings (single translation pipeline).
//!
//! One row per string: English, Portuguese, Spanish, German, French, Russian, Japanese.
//! The values are pushed into the Slint `Tr` global when the language changes.
//! Everything except English and Portuguese is a machine translation that still needs
//! a native-speaker review.

use std::sync::atomic::{AtomicU8, Ordering};

use super::i18n::Language;

macro_rules! ui_strings {
    ($(($key:ident, $setter:ident, $en:expr, $pt:expr, $es:expr, $de:expr, $fr:expr, $ru:expr, $ja:expr)),* $(,)?) => {
        pub struct UiStrings {
            $(pub $key: &'static str,)*
        }

        pub const EN_UI: UiStrings = UiStrings { $($key: $en,)* };
        pub const PT_UI: UiStrings = UiStrings { $($key: $pt,)* };
        pub const ES_UI: UiStrings = UiStrings { $($key: $es,)* };
        pub const DE_UI: UiStrings = UiStrings { $($key: $de,)* };
        pub const FR_UI: UiStrings = UiStrings { $($key: $fr,)* };
        pub const RU_UI: UiStrings = UiStrings { $($key: $ru,)* };
        pub const JA_UI: UiStrings = UiStrings { $($key: $ja,)* };

        /// Every table in `LANGUAGES` order, one `Vec` of (key, text) pairs per language.
        #[cfg(test)]
        pub fn entries() -> Vec<(&'static str, [&'static str; 7])> {
            vec![$((stringify!($key), [$en, $pt, $es, $de, $fr, $ru, $ja]),)*]
        }

        /// Pushes every string of `s` into the Slint `Tr` global.
        pub fn apply_to_window(ui: &crate::AppWindow, s: &UiStrings) {
            use slint::ComponentHandle;
            let g = ui.global::<crate::Tr>();
            $(g.$setter(s.$key.into());)*
        }
    };
}

ui_strings! {
    (gpu_auto, set_gpu_auto, "Automatic (recommended)", "Automático (recomendado)", "Automático (recomendado)", "Automatisch (empfohlen)", "Automatique (recommandé)", "Автоматически (рекомендуется)", "自動（推奨）"),
    (gpu_auto_desc, set_gpu_auto_desc, "Uses the GPU connected to the captured monitor, avoiding transfers between GPUs.", "Usa a GPU ligada ao monitor capturado, evitando transferências entre placas.", "Usa la GPU conectada al monitor capturado y evita transferencias entre GPU.", "Nutzt die GPU des aufgenommenen Monitors und vermeidet Übertragungen zwischen GPUs.", "Utilise le GPU relié à l’écran capturé, sans transfert entre GPU.", "Использует GPU захватываемого монитора без передачи между видеокартами.", "キャプチャするモニターに接続されたGPUを使用し、GPU間の転送を避けます。"),
    (gpu_cpu, set_gpu_cpu, "CPU (OpenH264)", "CPU (OpenH264)", "CPU (OpenH264)", "CPU (OpenH264)", "CPU (OpenH264)", "CPU (OpenH264)", "CPU（OpenH264）"),
    (gpu_cpu_desc, set_gpu_cpu_desc, "Compatibility option: encodes in software and uses more CPU.", "Opção de compatibilidade: codifica por software e usa mais CPU.", "Opción de compatibilidad: codifica por software y usa más CPU.", "Kompatibilitätsoption: Software-Kodierung mit höherer CPU-Last.", "Option de compatibilité : encodage logiciel, plus de charge CPU.", "Режим совместимости: программное кодирование с большей нагрузкой на CPU.", "互換モード。ソフトウェアでエンコードするためCPU負荷が増えます。"),
    (gpu_display_desc, set_gpu_display_desc, "DXGI captures this GPU's monitors. WGC captures windows or monitors on another GPU.", "DXGI captura os monitores desta GPU. WGC captura janelas ou monitores de outra placa.", "DXGI captura los monitores de esta GPU. WGC captura ventanas o monitores de otra GPU.", "DXGI erfasst Monitore dieser GPU. WGC erfasst Fenster oder Monitore anderer GPUs.", "DXGI capture les écrans de ce GPU. WGC capture les fenêtres ou écrans d’un autre GPU.", "DXGI захватывает мониторы этой GPU. WGC — окна и мониторы другой видеокарты.", "DXGIはこのGPUのモニターを、WGCはウィンドウや別のGPUのモニターをキャプチャします。"),
    (gpu_other_desc, set_gpu_other_desc, "WGC captures on this GPU; the image may be transferred between GPUs.", "WGC captura nesta GPU; a imagem pode ser transferida entre placas.", "WGC captura en esta GPU; la imagen puede transferirse entre GPU.", "WGC erfasst auf dieser GPU; Bilder können zwischen GPUs übertragen werden.", "WGC capture sur ce GPU ; l’image peut être transférée entre GPU.", "WGC захватывает на этой GPU; возможна передача изображения между видеокартами.", "WGCでこのGPUを使用します。GPU間で画像が転送される場合があります。"),
    (gpu_next_stream, set_gpu_next_stream, "Applies to the next screen share. If the selected GPU is unavailable, Automatic is used.", "Vale na próxima transmissão de tela. Se a GPU escolhida não estiver disponível, usa o automático.", "Se aplica a la próxima transmisión. Si la GPU no está disponible, se usa Automático.", "Gilt ab der nächsten Bildschirmübertragung. Ist die GPU nicht verfügbar, gilt Automatisch.", "S’applique au prochain partage d’écran. Si le GPU est indisponible, le mode automatique est utilisé.", "Применяется к следующей трансляции экрана. Если GPU недоступна, используется автоматический выбор.", "次の画面共有から適用されます。選択したGPUが使用できない場合は自動選択になります。"),
    (gpu_missing, set_gpu_missing, "Selected GPU unavailable", "GPU escolhida indisponível", "GPU seleccionada no disponible", "Gewählte GPU nicht verfügbar", "GPU sélectionné indisponible", "Выбранная GPU недоступна", "選択したGPUは使用できません"),
    (gpu_capture_now, set_gpu_capture_now, "Current capture", "Captura atual", "Captura actual", "Aktuelle Aufnahme", "Capture actuelle", "Текущий захват", "現在のキャプチャ"),
    (gpu_path, set_gpu_path, "GPU processing", "Processamento na GPU", "Procesamiento en GPU", "GPU-Verarbeitung", "Traitement GPU", "Обработка на GPU", "GPU処理"),
    (gpu_compat_path, set_gpu_compat_path, "Compatibility mode", "Modo de compatibilidade", "Modo de compatibilidad", "Kompatibilitätsmodus", "Mode de compatibilité", "Режим совместимости", "互換モード"),
    (gpu_gdi_help, set_gpu_gdi_help, "DXGI captures monitors; WGC captures windows or another GPU. The options below apply only if both fail and GDI is needed.", "DXGI captura monitores; WGC captura janelas ou outra GPU. As opções abaixo só valem se ambos falharem e for necessário usar GDI.", "DXGI captura monitores; WGC captura ventanas u otra GPU. Las opciones siguientes solo se usan si ambos fallan y se necesita GDI.", "DXGI erfasst Monitore, WGC Fenster oder eine andere GPU. Die folgenden Optionen gelten nur beim Rückgriff auf GDI.", "DXGI capture les écrans ; WGC les fenêtres ou un autre GPU. Ces options ne servent que si les deux échouent et que GDI est nécessaire.", "DXGI захватывает мониторы, WGC — окна или другую GPU. Эти настройки используются, только если оба способа не работают и нужен GDI.", "DXGIはモニター、WGCはウィンドウや別のGPUをキャプチャします。以下の設定は両方が失敗しGDIが必要な場合のみ使われます。"),
    (watch_stream, set_watch_stream, "Watch", "Assistir", "Ver", "Ansehen", "Regarder", "Смотреть", "視聴"),
    (web_share_start, set_web_share_start, "Share on web", "Compartilhar na web", "Compartir en la web", "Im Web teilen", "Partager sur le web", "Поделиться в браузере", "ウェブで共有"),
    (web_share_stop, set_web_share_stop, "Stop web", "Encerrar web", "Detener web", "Web beenden", "Arrêter le web", "Остановить веб", "ウェブ共有を停止"),
    (web_share_connecting, set_web_share_connecting, "Connecting…", "Conectando…", "Conectando…", "Verbinden…", "Connexion…", "Подключение…", "接続中…"),
    (web_share_copy, set_web_share_copy, "Copy link", "Copiar link", "Copiar enlace", "Link kopieren", "Copier le lien", "Копировать ссылку", "リンクをコピー"),
    (web_share_copied, set_web_share_copied, "Link copied", "Link copiado", "Enlace copiado", "Link kopiert", "Lien copié", "Ссылка скопирована", "コピーしました"),
    (web_share_title, set_web_share_title, "Watch on the web", "Assistir pela web", "Ver en la web", "Im Browser ansehen", "Regarder sur le web", "Смотреть в браузере", "ウェブで視聴"),
    (web_share_hint, set_web_share_hint, "Anyone with this link can watch in a browser, up to 3 at a time.", "Quem tiver este link pode assistir pelo navegador, até 3 pessoas ao mesmo tempo.", "Cualquiera con este enlace puede verlo en el navegador, hasta 3 a la vez.", "Wer diesen Link hat, kann im Browser zusehen, bis zu 3 gleichzeitig.", "Toute personne ayant ce lien peut regarder dans un navigateur, 3 à la fois maximum.", "Любой, у кого есть эта ссылка, может смотреть в браузере, до 3 человек одновременно.", "このリンクを知っている人はブラウザで視聴できます（同時に3人まで）。"),
    (web_share_watching, set_web_share_watching, "watching", "assistindo", "viendo", "sehen zu", "regardent", "смотрят", "人が視聴中"),
    (close, set_close, "Close", "Fechar", "Cerrar", "Schließen", "Fermer", "Закрыть", "閉じる"),
    (qr_generate, set_qr_generate, "Generate QR", "Gerar QR", "Generar QR", "QR generieren", "Générer le QR", "Создать QR", "QRを生成"),
    (qr_regenerate, set_qr_regenerate, "Re-generate QR Code", "Re-gerar QR Code", "Volver a generar código QR", "QR-Code neu generieren", "Régénérer le QR Code", "Создать QR-код заново", "QRコードを再生成"),
    (preview_reduced, set_preview_reduced, "Reduced preview to save resources", "Prévia reduzida para poupar recursos", "Vista previa reducida para ahorrar recursos", "Reduzierte Vorschau zur Ressourcenschonung", "Aperçu réduit pour économiser des ressources", "Уменьшенный предпросмотр для экономии ресурсов", "リソース節約のため簡易プレビュー"),
    (you_are_streaming, set_you_are_streaming, "You are streaming", "Você está transmitindo", "Estás transmitiendo", "Du streamst", "Vous diffusez", "Вы ведёте трансляцию", "配信中です"),
    (detached_badge, set_detached_badge, "DETACHED", "DESANEXADO", "DESANCLADO", "ABGEKOPPELT", "DÉTACHÉ", "ОТКРЕПЛЕНО", "分離中"),
    (live_badge, set_live_badge, "LIVE", "AO VIVO", "EN VIVO", "LIVE", "EN DIRECT", "В ЭФИРЕ", "ライブ"),
    (disable_intangible, set_disable_intangible, "Disable Intangible", "Desativar Intangível", "Desactivar intangible", "Unberührbar deaktivieren", "Désactiver l'intangibilité", "Отключить «Неосязаемый»", "「無形」を無効化"),
    (smart_commands, set_smart_commands, "DISCORD SMART COMMANDS", "COMANDOS INTELIGENTES DO DISCORD", "COMANDOS INTELIGENTES DE DISCORD", "DISCORD-SMARTBEFEHLE", "COMMANDES INTELLIGENTES DISCORD", "УМНЫЕ КОМАНДЫ DISCORD", "DISCORD スマートコマンド"),
    (click_autocomplete, set_click_autocomplete, "Click to autocomplete", "Clique para autocompletar", "Haz clic para autocompletar", "Zum Vervollständigen klicken", "Cliquez pour compléter automatiquement", "Нажмите для автозаполнения", "クリックして自動補完"),
    (manage_accounts, set_manage_accounts, "Manage Accounts", "Gerenciar Contas", "Administrar cuentas", "Konten verwalten", "Gérer les comptes", "Управление аккаунтами", "アカウント管理"),
    (manage_accounts_desc, set_manage_accounts_desc, "Quickly switch accounts or add a second saved account.", "Troque rapidamente de conta ou adicione uma segunda conta salva.", "Cambia rápidamente de cuenta o añade una segunda cuenta guardada.", "Wechsle schnell zwischen Konten oder füge ein zweites gespeichertes Konto hinzu.", "Changez rapidement de compte ou ajoutez un second compte enregistré.", "Быстро переключайтесь между аккаунтами или добавьте второй сохранённый аккаунт.", "アカウントをすばやく切り替えたり、2つ目の保存済みアカウントを追加できます。"),
    (account_connected, set_account_connected, "● Connected", "● Conectada", "● Conectada", "● Verbunden", "● Connecté", "● Подключён", "● 接続済み"),
    (account_sign_in, set_account_sign_in, "Sign in", "Entrar", "Iniciar sesión", "Anmelden", "Se connecter", "Войти", "ログイン"),
    (add_account, set_add_account, "+ Add Another Account", "+ Adicionar Outra Conta", "+ Añadir otra cuenta", "+ Weiteres Konto hinzufügen", "+ Ajouter un autre compte", "+ Добавить другой аккаунт", "+ 別のアカウントを追加"),
    (sign_out_all, set_sign_out_all, "Sign Out of All Accounts", "Sair de Todas as Contas", "Cerrar sesión en todas las cuentas", "Von allen Konten abmelden", "Se déconnecter de tous les comptes", "Выйти из всех аккаунтов", "すべてのアカウントからログアウト"),
    (update_available, set_update_available, "New Update Available!", "Nova Atualização Disponível!", "¡Nueva actualización disponible!", "Neues Update verfügbar!", "Nouvelle mise à jour disponible !", "Доступно новое обновление!", "新しいアップデートがあります！"),
    (dont_show_again, set_dont_show_again, "Don't show again", "Não mostrar mais", "No volver a mostrar", "Nicht mehr anzeigen", "Ne plus afficher", "Больше не показывать", "今後表示しない"),
    (remind_later, set_remind_later, "Remind Me Later", "Lembrar Depois", "Recordármelo luego", "Später erinnern", "Me le rappeler plus tard", "Напомнить позже", "後で通知"),
    (yes_update, set_yes_update, "Yes, Update", "Sim, Atualizar", "Sí, actualizar", "Ja, aktualisieren", "Oui, mettre à jour", "Да, обновить", "はい、更新します"),
    (share_your_screen, set_share_your_screen, "Share your screen", "Compartilhe sua tela", "Comparte tu pantalla", "Teile deinen Bildschirm", "Partagez votre écran", "Поделитесь экраном", "画面を共有"),
    (screens_windows, set_screens_windows, "Screens / Windows", "Telas / Janelas", "Pantallas / Ventanas", "Bildschirme / Fenster", "Écrans / Fenêtres", "Экраны / Окна", "画面 / ウィンドウ"),
    (screens, set_screens, "Screens", "Telas", "Pantallas", "Bildschirme", "Écrans", "Экраны", "画面"),
    (windows_tab, set_windows_tab, "Windows", "Janelas", "Ventanas", "Fenster", "Fenêtres", "Окна", "ウィンドウ"),
    (camera, set_camera, "Camera", "Câmera", "Cámara", "Kamera", "Caméra", "Камера", "カメラ"),
    (no_camera, set_no_camera, "No camera / webcam detected on the system.", "Nenhuma câmera / webcam detectada no sistema.", "No se detectó ninguna cámara / webcam en el sistema.", "Keine Kamera / Webcam im System gefunden.", "Aucune caméra / webcam détectée sur le système.", "В системе не обнаружена камера / веб-камера.", "システムにカメラ / ウェブカメラが見つかりません。"),
    (stream_quality, set_stream_quality, "STREAM QUALITY", "QUALIDADE DA TRANSMISSÃO", "CALIDAD DE LA TRANSMISIÓN", "STREAM-QUALITÄT", "QUALITÉ DE LA DIFFUSION", "КАЧЕСТВО ТРАНСЛЯЦИИ", "配信品質"),
    (resolution, set_resolution, "RESOLUTION", "RESOLUÇÃO", "RESOLUCIÓN", "AUFLÖSUNG", "RÉSOLUTION", "РАЗРЕШЕНИЕ", "解像度"),
    (frame_rate, set_frame_rate, "FRAME RATE (FPS)", "TAXA DE QUADROS (FPS)", "FRECUENCIA DE FOTOGRAMAS (FPS)", "BILDRATE (FPS)", "FRÉQUENCE D'IMAGES (FPS)", "ЧАСТОТА КАДРОВ (FPS)", "フレームレート (FPS)"),
    (webcam_native, set_webcam_native, "The webcam will stream using your device's native resolution and frame rate.", "A webcam transmitirá utilizando a resolução e taxa nativas do seu dispositivo.", "La webcam transmitirá con la resolución y la tasa de fotogramas nativas de tu dispositivo.", "Die Webcam überträgt mit der nativen Auflösung und Bildrate deines Geräts.", "La webcam diffusera avec la résolution et la fréquence d'images natives de votre appareil.", "Веб-камера будет передавать изображение с родным разрешением и частотой кадров устройства.", "ウェブカメラはデバイス本来の解像度とフレームレートで配信されます。"),
    (stream_pc_audio, set_stream_pc_audio, "Stream Computer / Window Audio", "Transmitir Áudio do Computador / Janela", "Transmitir audio del equipo / ventana", "Audio des Computers / Fensters übertragen", "Diffuser l'audio de l'ordinateur / de la fenêtre", "Транслировать звук компьютера / окна", "PC / ウィンドウの音声を配信"),
    (stream_pc_audio_desc, set_stream_pc_audio_desc, "Includes the app or game sound during the stream", "Inclui o som do aplicativo ou jogo durante a transmissão", "Incluye el sonido de la aplicación o el juego durante la transmisión", "Enthält den Ton der App oder des Spiels während der Übertragung", "Inclut le son de l'application ou du jeu pendant la diffusion", "Включает звук приложения или игры во время трансляции", "配信中にアプリやゲームの音を含めます"),
    (browser_stream, set_browser_stream, "Streaming Browser (Videos / DRM)", "Navegador para Transmissão (Vídeos / DRM)", "Navegador para transmisión (vídeos / DRM)", "Browser für Übertragung (Videos / DRM)", "Navigateur pour diffusion (vidéos / DRM)", "Браузер для трансляции (видео / DRM)", "配信用ブラウザ (動画 / DRM)"),
    (browser_stream_desc, set_browser_stream_desc, "Opens the browser without black-screen locks to watch with friends", "Abre o navegador sem travas de tela preta para assistir com amigos", "Abre el navegador sin bloqueos de pantalla negra para ver contenido con amigos", "Öffnet den Browser ohne Schwarzbild-Sperren, um gemeinsam mit Freunden zu schauen", "Ouvre le navigateur sans écran noir pour regarder avec des amis", "Открывает браузер без чёрного экрана, чтобы смотреть вместе с друзьями", "黒画面にならないブラウザを開き、友達と一緒に視聴できます"),
    (open, set_open, "Open", "Abrir", "Abrir", "Öffnen", "Ouvrir", "Открыть", "開く"),
    (direct_camera, set_direct_camera, "Direct Camera Stream", "Transmissão Direta de Câmera", "Transmisión directa de cámara", "Direkte Kameraübertragung", "Diffusion directe de la caméra", "Прямая трансляция с камеры", "カメラを直接配信"),
    (direct_camera_desc, set_direct_camera_desc, "Webcam video only (no system audio capture)", "Apenas vídeo da webcam (sem captura de áudio do sistema)", "Solo vídeo de la webcam (sin captura de audio del sistema)", "Nur Webcam-Video (ohne Systemaudio-Aufnahme)", "Vidéo de la webcam uniquement (sans capture de l'audio système)", "Только видео с веб-камеры (без захвата системного звука)", "ウェブカメラの映像のみ (システム音声はキャプチャしません)"),
    // dedupe: legacy Translations PT said "Voltar"; keeping the existing table row.
    (cancel, set_cancel, "Cancel", "Cancelar", "Cancelar", "Abbrechen", "Annuler", "Отмена", "キャンセル"),
    (incoming_call, set_incoming_call, "Incoming voice call", "Chamada de voz recebida", "Llamada de voz entrante", "Eingehender Sprachanruf", "Appel vocal entrant", "Входящий голосовой звонок", "音声通話の着信"),
    (answer, set_answer, "Answer", "Atender", "Contestar", "Annehmen", "Répondre", "Ответить", "応答"),
    (decline, set_decline, "Decline", "Recusar", "Rechazar", "Ablehnen", "Refuser", "Отклонить", "拒否"),
    (detached_title, set_detached_title, "Litecord - Detached Stream", "Litecord - Transmissão Desanexada", "Litecord - Transmisión desanclada", "Litecord - Abgekoppelte Übertragung", "Litecord - Diffusion détachée", "Litecord - Откреплённая трансляция", "Litecord - 分離した配信"),
    (settings_header, set_settings_header, "SETTINGS", "CONFIGURAÇÕES", "AJUSTES", "EINSTELLUNGEN", "PARAMÈTRES", "НАСТРОЙКИ", "設定"),
    (output_device, set_output_device, "Output Device", "Dispositivo de Saída", "Dispositivo de salida", "Ausgabegerät", "Périphérique de sortie", "Устройство вывода", "出力デバイス"),
    (output_hint, set_output_hint, "For the best positional fidelity and to avoid echo or sound leaking in group calls, use stereo headphones.", "Para obter a melhor fidelidade posicional e evitar eco ou vazamento de som nas chamadas em grupo, utilize fones de ouvido estéreo.", "Para obtener la mejor fidelidad posicional y evitar eco o fugas de sonido en llamadas grupales, usa auriculares estéreo.", "Für die beste Positionswiedergabe und um Echo oder Tonaustritt in Gruppenanrufen zu vermeiden, verwende Stereo-Kopfhörer.", "Pour une meilleure fidélité positionnelle et éviter l'écho ou les fuites de son lors des appels de groupe, utilisez un casque stéréo.", "Для лучшей позиционной точности и чтобы избежать эха и утечки звука в групповых звонках, используйте стереонаушники.", "定位の再現性を高め、グループ通話でのエコーや音漏れを防ぐため、ステレオヘッドホンの使用をおすすめします。"),
    (input_device, set_input_device, "Input Device", "Dispositivo de Entrada", "Dispositivo de entrada", "Eingabegerät", "Périphérique d'entrée", "Устройство ввода", "入力デバイス"),
    (input_hint, set_input_hint, "Select your main microphone or headset. We recommend using the default device or a dedicated noise-cancelling microphone.", "Selecione o microfone principal ou headset. Recomendamos usar o dispositivo padrão ou o microfone dedicado com cancelamento de ruído.", "Selecciona tu micrófono principal o auriculares. Recomendamos usar el dispositivo predeterminado o un micrófono dedicado con cancelación de ruido.", "Wähle dein Hauptmikrofon oder Headset. Wir empfehlen das Standardgerät oder ein eigenes Mikrofon mit Geräuschunterdrückung.", "Sélectionnez votre microphone principal ou votre casque. Nous recommandons le périphérique par défaut ou un microphone dédié avec réduction de bruit.", "Выберите основной микрофон или гарнитуру. Рекомендуем использовать устройство по умолчанию или отдельный микрофон с шумоподавлением.", "メインのマイクまたはヘッドセットを選択してください。既定のデバイス、またはノイズキャンセリング対応の専用マイクをおすすめします。"),
    (input_volume_title, set_input_volume_title, "INPUT VOLUME (MICROPHONE GAIN)", "VOLUME DE ENTRADA (GANHO DO MICROFONE)", "VOLUMEN DE ENTRADA (GANANCIA DEL MICRÓFONO)", "EINGABELAUTSTÄRKE (MIKROFONVERSTÄRKUNG)", "VOLUME D'ENTRÉE (GAIN DU MICROPHONE)", "ГРОМКОСТЬ ВВОДА (УСИЛЕНИЕ МИКРОФОНА)", "入力音量 (マイクゲイン)"),
    (mic_gain, set_mic_gain, "Microphone Gain", "Ganho do Microfone", "Ganancia del micrófono", "Mikrofonverstärkung", "Gain du microphone", "Усиление микрофона", "マイクゲイン"),
    (mic_gain_desc, set_mic_gain_desc, "Adjust your microphone gain. Values above 100% (up to 200%) amplify your voice signal for the other users in the room.", "Ajuste o ganho do seu microfone. Valores acima de 100% (até 200%) amplificam o sinal da sua voz para os outros usuários na sala.", "Ajusta la ganancia de tu micrófono. Los valores superiores al 100 % (hasta el 200 %) amplifican tu voz para los demás usuarios de la sala.", "Passe die Mikrofonverstärkung an. Werte über 100 % (bis 200 %) verstärken deine Stimme für die anderen Nutzer im Raum.", "Réglez le gain de votre microphone. Les valeurs supérieures à 100 % (jusqu'à 200 %) amplifient votre voix pour les autres utilisateurs du salon.", "Настройте усиление микрофона. Значения выше 100 % (до 200 %) усиливают ваш голос для остальных участников комнаты.", "マイクのゲインを調整します。100%を超える値 (最大200%) にすると、ルーム内の他のユーザーに届く声が大きくなります。"),
    (sensitivity, set_sensitivity, "Sensitivity (VAD)", "Sensibilidade (VAD)", "Sensibilidad (VAD)", "Empfindlichkeit (VAD)", "Sensibilité (VAD)", "Чувствительность (VAD)", "感度 (VAD)"),
    (sensitivity_desc, set_sensitivity_desc, "Speak normally. Adjust the control so your voice goes past the white mark, while noises such as a mechanical keyboard and breathing stay below it.", "Fale normalmente. Ajuste o controle para que sua voz ultrapasse a marca branca, mas ruídos como teclado mecânico e respiração fiquem abaixo.", "Habla con normalidad. Ajusta el control para que tu voz supere la marca blanca, pero ruidos como el teclado mecánico o la respiración queden por debajo.", "Sprich normal. Stelle den Regler so ein, dass deine Stimme die weiße Markierung überschreitet, Geräusche wie mechanische Tastatur und Atmen aber darunter bleiben.", "Parlez normalement. Réglez le curseur pour que votre voix dépasse le repère blanc, tandis que les bruits comme le clavier mécanique et la respiration restent en dessous.", "Говорите как обычно. Настройте ползунок так, чтобы голос превышал белую отметку, а шумы вроде механической клавиатуры и дыхания оставались ниже.", "普通に話してください。声が白い目印を超え、メカニカルキーボードや呼吸音などのノイズが下回るようにスライダーを調整します。"),
    (loopback_test, set_loopback_test, "Loopback Test", "Teste de Retorno (Loopback)", "Prueba de retorno (loopback)", "Rückkanal-Test (Loopback)", "Test de retour (loopback)", "Тест обратной связи (loopback)", "ループバックテスト"),
    (loopback_test_desc, set_loopback_test_desc, "Lets you hear your processed voice in real time with Litecord's filter through your headphones to calibrate volume and clarity before matches.", "Permite escutar a sua voz tratada em tempo real com o filtro do Litecord nos seus fones para calibrar volume e clareza antes das partidas.", "Te permite escuchar tu voz procesada en tiempo real con el filtro de Litecord en tus auriculares para calibrar volumen y claridad antes de las partidas.", "Du hörst deine verarbeitete Stimme in Echtzeit mit dem Litecord-Filter über deine Kopfhörer, um Lautstärke und Klarheit vor Spielen einzustellen.", "Vous permet d'entendre votre voix traitée en temps réel avec le filtre de Litecord dans votre casque, pour calibrer le volume et la clarté avant les parties.", "Позволяет слышать свой обработанный голос в реальном времени с фильтром Litecord в наушниках, чтобы настроить громкость и чёткость перед матчами.", "Litecordのフィルターを通した自分の声をヘッドホンでリアルタイムに聞き、試合前に音量と明瞭さを調整できます。"),
    (ducking, set_ducking, "Smart Voice Ducking", "Atenuação de Voz Inteligente (Ducking)", "Atenuación inteligente de voz (ducking)", "Intelligente Sprachabsenkung (Ducking)", "Atténuation vocale intelligente (ducking)", "Умное приглушение голоса (ducking)", "スマート音声ダッキング"),
    (ducking_active, set_ducking_active, "ACTIVE (P:1 to P:5)", "ATIVO (P:1 a P:5)", "ACTIVO (P:1 a P:5)", "AKTIV (P:1 bis P:5)", "ACTIF (P:1 à P:5)", "АКТИВНО (P:1–P:5)", "有効 (P:1〜P:5)"),
    (ducking_desc, set_ducking_desc, "The Opus PLC audio engine automatically lowers other members' sound by 12 dB when priority tactical leaders speak.", "O motor de áudio Opus PLC reduz automaticamente em 12dB o som dos outros membros quando líderes táticos com prioridade falam.", "El motor de audio Opus PLC reduce automáticamente 12 dB el sonido de los demás miembros cuando hablan líderes tácticos con prioridad.", "Die Opus-PLC-Audio-Engine senkt die Lautstärke der anderen Mitglieder automatisch um 12 dB, wenn taktische Anführer mit Priorität sprechen.", "Le moteur audio Opus PLC réduit automatiquement de 12 dB le son des autres membres lorsque des leaders tactiques prioritaires parlent.", "Аудиодвижок Opus PLC автоматически снижает громкость остальных участников на 12 дБ, когда говорят приоритетные тактические лидеры.", "Opus PLCオーディオエンジンは、優先度の高いリーダーが話すと、他のメンバーの音を自動的に12dB下げます。"),
    (competitive_edge, set_competitive_edge, "Competitive Edge", "Vantagem Competitiva", "Ventaja competitiva", "Wettbewerbsvorteil", "Avantage compétitif", "Соревновательное преимущество", "競技での優位性"),
    (competitive_edge_desc, set_competitive_edge_desc, "Eliminates the cacophony in decisive FPS/MOBA moments, making sure critical callouts stand out instantly.", "Elimina a cacofonia em momentos decisivos de jogos FPS/MOBA, garantindo que callouts críticos se sobressaiam instantaneamente.", "Elimina la cacofonía en los momentos decisivos de los juegos FPS/MOBA y hace que los avisos críticos destaquen al instante.", "Beseitigt das Stimmengewirr in entscheidenden Momenten von FPS/MOBA-Spielen, damit kritische Ansagen sofort hervorstechen.", "Élimine la cacophonie lors des moments décisifs des jeux FPS/MOBA et fait ressortir instantanément les annonces critiques.", "Устраняет какофонию в решающие моменты FPS/MOBA-игр, так что важные подсказки сразу выделяются.", "FPS/MOBAの決定的な場面での騒がしさを抑え、重要なコールがすぐに聞き取れるようにします。"),
    (voice_processing, set_voice_processing, "VOICE PROCESSING", "PROCESSAMENTO DE VOZ", "PROCESAMIENTO DE VOZ", "SPRACHVERARBEITUNG", "TRAITEMENT DE LA VOIX", "ОБРАБОТКА ГОЛОСА", "音声処理"),
    (noise_suppression, set_noise_suppression, "Noise Suppression", "Supressão de Ruído", "Supresión de ruido", "Rauschunterdrückung", "Suppression du bruit", "Шумоподавление", "ノイズ抑制"),
    (noise_suppression_desc, set_noise_suppression_desc, "Filter background noise from your microphone using advanced deep learning or standard filtering.", "Filtra ruídos de fundo do microfone usando aprendizado de máquina inteligente ou filtragem padrão.", "Filtra el ruido de fondo de tu micrófono mediante aprendizaje automático o filtrado estándar.", "Filtert Hintergrundgeräusche mit maschinellem Lernen oder Standardfiltern aus dem Mikrofon.", "Filtre les bruits de fond du microphone grâce à l'apprentissage automatique ou au filtrage standard.", "Фильтрует фоновый шум микрофона с помощью машинного обучения или стандартной фильтрации.", "深層学習や標準フィルタでマイクの背景ノイズを除去します。"),
    (echo_cancellation, set_echo_cancellation, "Echo Cancellation", "Cancelamento de Eco", "Cancelación de eco", "Echounterdrückung", "Annulation de l'écho", "Эхоподавление", "エコーキャンセラー"),
    (echo_cancellation_desc, set_echo_cancellation_desc, "Prevents audio from your speakers from leaking back into your microphone (AEC).", "Evita que o áudio dos alto-falantes retorne para o microfone (AEC).", "Evita que el audio de los altavoces vuelva a entrar por el micrófono (AEC).", "Verhindert, dass der Ton der Lautsprecher wieder in das Mikrofon gelangt (AEC).", "Empêche le son de vos haut-parleurs de repasser dans votre microphone (AEC).", "Предотвращает попадание звука из динамиков обратно в микрофон (AEC).", "スピーカーの音声がマイクに回り込むのを防ぎます (AEC)。"),
    (auto_gain_control, set_auto_gain_control, "Automatic Gain Control (AGC)", "Controle Automático de Ganho (AGC)", "Control automático de ganancia (AGC)", "Automatische Lautstärkeregelung (AGC)", "Contrôle automatique du gain (AGC)", "Автоматическая регулировка усиления (АРУ)", "自動ゲイン制御 (AGC)"),
    (auto_gain_control_desc, set_auto_gain_control_desc, "Automatically normalizes and boosts your microphone volume to prevent sudden clipping or inaudible whispering.", "Normaliza e amplifica a voz automaticamente, evitando clipping ou sussurros inaudíveis.", "Normaliza y amplifica la voz automáticamente, evitando distorsiones o susurros inaudibles.", "Normalisiert und verstärkt die Stimme automatisch, um Übersteuern oder unhörbares Flüstern zu verhindern.", "Normalise et amplifie automatiquement la voix pour éviter la saturation ou les chuchotements inaudibles.", "Автоматически выравнивает и усиливает громкость микрофона, предотвращая клиппинг или неслышный шепот.", "マイク音量を自動で均一化・増幅し、音割れや聞き取れないささやき声を防ぎます。"),
    (select_language, set_select_language, "SELECT THE CLIENT LANGUAGE", "SELECIONE O IDIOMA DO CLIENTE", "SELECCIONA EL IDIOMA DEL CLIENTE", "SPRACHE DES CLIENTS AUSWÄHLEN", "SÉLECTIONNEZ LA LANGUE DU CLIENT", "ВЫБЕРИТЕ ЯЗЫК КЛИЕНТА", "クライアントの言語を選択"),
    (instant_switch, set_instant_switch, "Instant Switch", "Troca Instantânea", "Cambio instantáneo", "Sofortiger Wechsel", "Changement instantané", "Мгновенное переключение", "即時切り替え"),
    (instant_switch_desc, set_instant_switch_desc, "Language changes are applied in real time across all windows without restarting Litecord.", "A alteração de idioma é aplicada em tempo real em todas as janelas sem necessidade de reiniciar o Litecord.", "El cambio de idioma se aplica en tiempo real en todas las ventanas sin reiniciar Litecord.", "Sprachänderungen werden in Echtzeit in allen Fenstern übernommen, ohne Litecord neu zu starten.", "Le changement de langue s'applique en temps réel à toutes les fenêtres, sans redémarrer Litecord.", "Смена языка применяется в реальном времени во всех окнах без перезапуска Litecord.", "言語の変更は、Litecordを再起動せずにすべてのウィンドウへ即時に反映されます。"),
    (auto_detect, set_auto_detect, "Automatic Detection", "Detecção Automática", "Detección automática", "Automatische Erkennung", "Détection automatique", "Автоопределение", "自動検出"),
    (auto_detect_desc, set_auto_detect_desc, "The 'Auto' option detects the default region and language of your operating system (Windows/Linux).", "A opção 'Auto' identifica a região e linguagem padrão do seu sistema operacional (Windows/Linux).", "La opción «Auto» detecta la región y el idioma predeterminados de tu sistema operativo (Windows/Linux).", "Die Option „Auto“ erkennt die Standardregion und -sprache deines Betriebssystems (Windows/Linux).", "L'option « Auto » détecte la région et la langue par défaut de votre système d'exploitation (Windows/Linux).", "Параметр «Авто» определяет регион и язык вашей операционной системы по умолчанию (Windows/Linux).", "「自動」を選ぶと、OS (Windows/Linux) の既定の地域と言語を検出します。"),
    (keys_tip, set_keys_tip, "Key Tip", "Dica de Teclas", "Consejo de teclas", "Tastentipp", "Astuce de touches", "Совет по клавишам", "キーのヒント"),
    (keys_tip_desc, set_keys_tip_desc, "You can use keys such as F1-F12, NumPad keys, or combinations with Ctrl, Shift and Alt to avoid conflicts.", "Você pode usar teclas como F1-F12, teclas do NumPad, ou combinações com Ctrl, Shift e Alt para evitar conflitos.", "Puedes usar teclas como F1-F12, teclas del teclado numérico o combinaciones con Ctrl, Shift y Alt para evitar conflictos.", "Du kannst Tasten wie F1–F12, Ziffernblock-Tasten oder Kombinationen mit Strg, Umschalt und Alt verwenden, um Konflikte zu vermeiden.", "Vous pouvez utiliser des touches comme F1-F12, les touches du pavé numérique ou des combinaisons avec Ctrl, Maj et Alt pour éviter les conflits.", "Можно использовать клавиши F1–F12, клавиши цифровой панели или сочетания с Ctrl, Shift и Alt, чтобы избежать конфликтов.", "競合を避けるため、F1〜F12、テンキー、またはCtrl・Shift・Altとの組み合わせを使用できます。"),
    (encoder_title, set_encoder_title, "VIDEO ENCODER (H.264)", "CODIFICADOR DE VÍDEO (H.264)", "CODIFICADOR DE VÍDEO (H.264)", "VIDEO-ENCODER (H.264)", "ENCODEUR VIDÉO (H.264)", "ВИДЕОКОДЕР (H.264)", "ビデオエンコーダー (H.264)"),
    (encoder_desc, set_encoder_desc, "Select the GPU or CPU acceleration engine used to encode screen streams.", "Selecione o motor de aceleração por GPU ou CPU para codificar as transmissões de tela.", "Selecciona el motor de aceleración por GPU o CPU para codificar las transmisiones de pantalla.", "Wähle die GPU- oder CPU-Beschleunigung zum Kodieren von Bildschirmübertragungen.", "Sélectionnez le moteur d'accélération GPU ou CPU pour encoder les diffusions d'écran.", "Выберите GPU- или CPU-ускорение для кодирования трансляций экрана.", "画面配信のエンコードに使うGPUまたはCPUのアクセラレーションを選択します。"),
    (appearance, set_appearance, "APPEARANCE", "APARÊNCIA", "APARIENCIA", "DARSTELLUNG", "APPARENCE", "ВНЕШНИЙ ВИД", "外観"),
    (light_theme, set_light_theme, "Light theme", "Tema claro", "Tema claro", "Helles Design", "Thème clair", "Светлая тема", "ライトテーマ"),
    (light_theme_desc, set_light_theme_desc, "Switches the whole interface between the light and dark palettes.", "Alterna toda a interface entre as paletas clara e escura.", "Cambia toda la interfaz entre las paletas clara y oscura.", "Wechselt die gesamte Oberfläche zwischen hellem und dunklem Farbschema.", "Bascule toute l'interface entre les palettes claire et sombre.", "Переключает весь интерфейс между светлой и тёмной палитрами.", "インターフェース全体のライト/ダークの配色を切り替えます。"),
    (capture_method, set_capture_method, "SCREEN AND WINDOW CAPTURE METHOD", "MÉTODO DE CAPTURA DE TELA E JANELAS", "MÉTODO DE CAPTURA DE PANTALLA Y VENTANAS", "METHODE ZUR BILDSCHIRM- UND FENSTERAUFNAHME", "MÉTHODE DE CAPTURE D'ÉCRAN ET DE FENÊTRES", "СПОСОБ ЗАХВАТА ЭКРАНА И ОКОН", "画面とウィンドウのキャプチャ方法"),
    (capture_method_desc, set_capture_method_desc, "Defines how Litecord captures the visual content of the operating system.", "Define como o Litecord captura o conteúdo visual do sistema operacional.", "Define cómo Litecord captura el contenido visual del sistema operativo.", "Legt fest, wie Litecord den visuellen Inhalt des Betriebssystems aufnimmt.", "Définit comment Litecord capture le contenu visuel du système d'exploitation.", "Определяет, как Litecord захватывает визуальное содержимое операционной системы.", "LitecordがOSの画面内容をキャプチャする方法を設定します。"),
    (loopback_audio_title, set_loopback_audio_title, "STREAM AUDIO (SYSTEM LOOPBACK)", "ÁUDIO DA TRANSMISSÃO (LOOPBACK DO SISTEMA)", "AUDIO DE LA TRANSMISIÓN (LOOPBACK DEL SISTEMA)", "ÜBERTRAGUNGSAUDIO (SYSTEM-LOOPBACK)", "AUDIO DE LA DIFFUSION (LOOPBACK SYSTÈME)", "ЗВУК ТРАНСЛЯЦИИ (СИСТЕМНЫЙ LOOPBACK)", "配信音声 (システムループバック)"),
    (loopback_audio_desc, set_loopback_audio_desc, "Defines the technology used to capture the internal sound of games, videos and the system.", "Define a tecnologia de captura do som interno de jogos, vídeos e do sistema.", "Define la tecnología para capturar el sonido interno de juegos, vídeos y del sistema.", "Legt die Technik zur Aufnahme des internen Tons von Spielen, Videos und System fest.", "Définit la technologie de capture du son interne des jeux, des vidéos et du système.", "Определяет технологию захвата внутреннего звука игр, видео и системы.", "ゲーム、動画、システムの内部音声をキャプチャする方式を設定します。"),
    (stream_ui_title, set_stream_ui_title, "STREAM NOTICES AND INTERFACE", "AVISOS E INTERFACE DE TRANSMISSÃO", "AVISOS E INTERFAZ DE TRANSMISIÓN", "HINWEISE UND OBERFLÄCHE DER ÜBERTRAGUNG", "AVIS ET INTERFACE DE DIFFUSION", "УВЕДОМЛЕНИЯ И ИНТЕРФЕЙС ТРАНСЛЯЦИИ", "配信の通知とインターフェース"),
    (economy_notice, set_economy_notice, "Reduced Preview Notice", "Aviso de Prévia Econômica", "Aviso de vista previa reducida", "Hinweis auf reduzierte Vorschau", "Avis d'aperçu réduit", "Уведомление об уменьшенном предпросмотре", "簡易プレビューの通知"),
    (economy_notice_desc, set_economy_notice_desc, "Shows the warning pill in your own preview when the stream starts, telling you the frame rate is reduced to save resources.", "Exibe a pílula de aviso na pré-visualização própria ao iniciar a transmissão informando sobre a taxa de quadros reduzida para economizar recursos.", "Muestra la etiqueta de aviso en tu propia vista previa al iniciar la transmisión, indicando que la tasa de fotogramas se reduce para ahorrar recursos.", "Zeigt beim Start der Übertragung in deiner eigenen Vorschau ein Hinweis-Label, dass die Bildrate zur Ressourcenschonung reduziert ist.", "Affiche l'étiquette d'avertissement dans votre propre aperçu au démarrage de la diffusion, indiquant que la fréquence d'images est réduite pour économiser des ressources.", "Показывает метку-предупреждение в вашем предпросмотре при запуске трансляции: частота кадров снижена для экономии ресурсов.", "配信開始時、自分のプレビューに、リソース節約のためフレームレートを下げている旨の通知を表示します。"),
    (perf_monitor, set_perf_monitor, "Performance Monitor (CPU / RAM / GPU)", "Monitor de Desempenho (CPU / RAM / GPU)", "Monitor de rendimiento (CPU / RAM / GPU)", "Leistungsmonitor (CPU / RAM / GPU)", "Moniteur de performances (CPU / RAM / GPU)", "Монитор производительности (CPU / RAM / GPU)", "パフォーマンスモニター (CPU / RAM / GPU)"),
    (perf_monitor_desc, set_perf_monitor_desc, "Shows live RAM, CPU and GPU usage in the application title bar next to the settings.", "Exibe em tempo real o percentual de uso de RAM, CPU e GPU na barra de título do aplicativo ao lado das configurações.", "Muestra en tiempo real el uso de RAM, CPU y GPU en la barra de título de la aplicación, junto a los ajustes.", "Zeigt die RAM-, CPU- und GPU-Auslastung live in der Titelleiste der Anwendung neben den Einstellungen an.", "Affiche en temps réel l'utilisation de la RAM, du CPU et du GPU dans la barre de titre de l'application, à côté des paramètres.", "Показывает в реальном времени загрузку RAM, CPU и GPU в строке заголовка приложения рядом с настройками.", "RAM、CPU、GPUの使用率を、設定の横にあるアプリのタイトルバーにリアルタイムで表示します。"),
    (net_diag_title, set_net_diag_title, "Network diagnostics", "Diagnóstico de rede", "Diagnóstico de red", "Netzwerkdiagnose", "Diagnostic réseau", "Диагностика сети", "ネットワーク診断"),
    (net_diag_desc, set_net_diag_desc, "Checks whether screen sharing with someone on another network can connect directly from here. “Copy report” puts the details on the clipboard for a bug report.", "Verifica se a transmissão de tela com alguém de outra rede consegue se conectar direto daqui. “Copiar relatório” põe os detalhes na área de transferência para um relato de problema.", "Comprueba si la transmisión de pantalla con alguien de otra red puede conectarse directamente desde aquí. “Copiar informe” copia los detalles al portapapeles para un reporte de error.", "Prüft, ob die Bildschirmübertragung mit jemandem in einem anderen Netzwerk von hier aus direkt verbinden kann. „Bericht kopieren“ legt die Details für einen Fehlerbericht in die Zwischenablage.", "Vérifie si le partage d'écran avec quelqu'un sur un autre réseau peut se connecter directement depuis ici. « Copier le rapport » place les détails dans le presse-papiers pour un rapport de bug.", "Проверяет, может ли трансляция экрана с человеком из другой сети подключиться отсюда напрямую. «Скопировать отчёт» помещает подробности в буфер обмена для сообщения об ошибке.", "別のネットワークの相手との画面共有が、ここから直接接続できるかを確認します。「レポートをコピー」で詳細をクリップボードにコピーし、不具合報告に使えます。"),
    (net_diag_run, set_net_diag_run, "Test network", "Testar rede", "Probar red", "Netzwerk testen", "Tester le réseau", "Проверить сеть", "ネットワークをテスト"),
    (net_diag_running, set_net_diag_running, "Testing...", "Testando...", "Probando...", "Wird getestet...", "Test en cours...", "Проверка...", "テスト中..."),
    (net_diag_copy, set_net_diag_copy, "Copy report", "Copiar relatório", "Copiar informe", "Bericht kopieren", "Copier le rapport", "Скопировать отчёт", "レポートをコピー"),
    (net_diag_copied, set_net_diag_copied, "Copied!", "Copiado!", "¡Copiado!", "Kopiert!", "Copié !", "Скопировано!", "コピーしました"),
    (net_diag_ok, set_net_diag_ok, "Your network allows direct connections: screen sharing with other networks should work.", "Sua rede permite conexão direta: transmissões com pessoas de outras redes devem funcionar.", "Tu red permite conexiones directas: la transmisión con otras redes debería funcionar.", "Dein Netzwerk erlaubt direkte Verbindungen: Bildschirmübertragungen mit anderen Netzwerken sollten funktionieren.", "Votre réseau permet les connexions directes : le partage d'écran avec d'autres réseaux devrait fonctionner.", "Ваша сеть допускает прямые подключения: трансляции с другими сетями должны работать.", "このネットワークは直接接続できます。他のネットワークとの画面共有は動作するはずです。"),
    (net_diag_symmetric, set_net_diag_symmetric, "Your router or provider uses symmetric NAT: screen sharing across the internet may not connect (the local network works).", "Seu roteador ou operadora usa NAT simétrico: transmissões pela internet podem não conectar (na rede local funcionam).", "Tu router o proveedor usa NAT simétrico: la transmisión por internet puede no conectar (la red local funciona).", "Dein Router oder Anbieter nutzt symmetrisches NAT: Bildschirmübertragungen über das Internet verbinden eventuell nicht (im lokalen Netz funktioniert es).", "Votre routeur ou fournisseur utilise un NAT symétrique : le partage d'écran par Internet risque de ne pas se connecter (le réseau local fonctionne).", "Ваш роутер или провайдер использует симметричный NAT: трансляции через интернет могут не подключаться (в локальной сети работают).", "ルーターまたはプロバイダーが対称型NATを使用しています。インターネット経由の画面共有は接続できない場合があります（ローカルネットワークでは動作します）。"),
    (net_diag_varying_ip, set_net_diag_varying_ip, "Your provider shows a different public address per server (carrier-grade NAT): screen sharing across the internet may not connect.", "Sua operadora mostra um endereço público diferente para cada servidor (CGNAT): transmissões pela internet podem não conectar.", "Tu proveedor muestra una dirección pública distinta para cada servidor (CGNAT): la transmisión por internet puede no conectar.", "Dein Anbieter zeigt pro Server eine andere öffentliche Adresse (CGNAT): Bildschirmübertragungen über das Internet verbinden eventuell nicht.", "Votre fournisseur présente une adresse publique différente pour chaque serveur (CGNAT) : le partage d'écran par Internet risque de ne pas se connecter.", "Провайдер показывает разные публичные адреса для разных серверов (CGNAT): трансляции через интернет могут не подключаться.", "プロバイダーがサーバーごとに異なる公開アドレスを使っています（CGNAT）。インターネット経由の画面共有は接続できない場合があります。"),
    (net_diag_udp_blocked, set_net_diag_udp_blocked, "No test server answered: UDP looks blocked on this network, so screen sharing across the internet will not connect.", "Nenhum servidor de teste respondeu: o UDP parece bloqueado nesta rede, então transmissões pela internet não vão conectar.", "Ningún servidor de prueba respondió: UDP parece bloqueado en esta red, así que la transmisión por internet no conectará.", "Kein Testserver hat geantwortet: UDP scheint in diesem Netzwerk blockiert zu sein, Bildschirmübertragungen über das Internet werden nicht verbinden.", "Aucun serveur de test n'a répondu : l'UDP semble bloqué sur ce réseau, le partage d'écran par Internet ne pourra pas se connecter.", "Ни один тестовый сервер не ответил: UDP в этой сети, похоже, заблокирован, поэтому трансляции через интернет не подключатся.", "テストサーバーが応答しませんでした。このネットワークではUDPがブロックされているようで、インターネット経由の画面共有は接続できません。"),
    (net_diag_signaling_down, set_net_diag_signaling_down, "The signaling server did not answer, so peers cannot find each other over the internet.", "O servidor de sinalização não respondeu, então os participantes não se encontram pela internet.", "El servidor de señalización no respondió, así que los participantes no pueden encontrarse por internet.", "Der Signalisierungsserver hat nicht geantwortet, daher finden sich die Teilnehmer nicht über das Internet.", "Le serveur de signalisation n'a pas répondu, les participants ne peuvent donc pas se trouver par Internet.", "Сервер сигнализации не ответил, поэтому участники не могут найти друг друга через интернет.", "シグナリングサーバーが応答しなかったため、インターネット経由で参加者同士が見つかりません。"),
    (gpu_render, set_gpu_render, "GPU rendering", "Renderização pela GPU", "Renderizado por GPU", "GPU-Rendering", "Rendu par le GPU", "Отрисовка на GPU", "GPUレンダリング"),
    (gpu_render_desc, set_gpu_render_desc, "Draws the interface with the graphics card. Off (default), the CPU draws it, which costs almost nothing while the app is idle and does not share the GPU with your game. Takes effect after restarting Litecord.", "Desenha a interface com a placa de vídeo. Desligado (padrão), a CPU desenha a interface, o que quase não custa nada com o app parado e não divide a GPU com o seu jogo. Vale depois de reiniciar o Litecord.", "Dibuja la interfaz con la tarjeta gráfica. Desactivado (predeterminado), la CPU dibuja la interfaz, lo que casi no consume nada con la aplicación inactiva y no comparte la GPU con tu juego. Se aplica al reiniciar Litecord.", "Zeichnet die Oberfläche mit der Grafikkarte. Aus (Standard) zeichnet die CPU sie, was im Leerlauf fast nichts kostet und die GPU nicht mit deinem Spiel teilt. Wirkt nach einem Neustart von Litecord.", "Dessine l'interface avec la carte graphique. Désactivé (par défaut), le CPU la dessine, ce qui ne coûte presque rien quand l'application est inactive et ne partage pas le GPU avec votre jeu. Prend effet après le redémarrage de Litecord.", "Отрисовывает интерфейс видеокартой. Если выключено (по умолчанию), интерфейс рисует процессор: в простое это почти ничего не стоит и не делит видеокарту с игрой. Вступает в силу после перезапуска Litecord.", "グラフィックカードでインターフェースを描画します。オフ（既定）ではCPUが描画し、待機中はほとんど負荷がかからず、ゲームとGPUを共有しません。Litecordの再起動後に有効になります。"),
    (e2ee_title, set_e2ee_title, "E2EE Video & Sound Encryption", "Criptografia E2EE de Vídeo & Som", "Cifrado E2EE de vídeo y sonido", "E2EE-Verschlüsselung für Video & Ton", "Chiffrement E2EE de la vidéo et du son", "Сквозное шифрование (E2EE) видео и звука", "映像と音声のE2EE暗号化"),
    (aes_active, set_aes_active, "AES-256-GCM ACTIVE", "AES-256-GCM ATIVO", "AES-256-GCM ACTIVO", "AES-256-GCM AKTIV", "AES-256-GCM ACTIF", "AES-256-GCM АКТИВНО", "AES-256-GCM 有効"),
    (e2ee_desc, set_e2ee_desc, "Every 60 FPS frame and the loopback audio are encrypted with 256-bit symmetric keys using AES-NI hardware acceleration.", "Cada quadro a 60 FPS e áudio loopback é criptografado com chaves simétricas de 256 bits via aceleração de hardware AES-NI.", "Cada fotograma a 60 FPS y el audio loopback se cifran con claves simétricas de 256 bits mediante aceleración por hardware AES-NI.", "Jedes Bild mit 60 FPS und das Loopback-Audio werden mit symmetrischen 256-Bit-Schlüsseln per AES-NI-Hardwarebeschleunigung verschlüsselt.", "Chaque image à 60 FPS et l'audio loopback sont chiffrés avec des clés symétriques de 256 bits grâce à l'accélération matérielle AES-NI.", "Каждый кадр при 60 FPS и loopback-звук шифруются симметричными 256-битными ключами с аппаратным ускорением AES-NI.", "60FPSの各フレームとループバック音声は、AES-NIのハードウェアアクセラレーションにより256ビットの共通鍵で暗号化されます。"),
    (lan_protect, set_lan_protect, "Wi-Fi & LAN Protection", "Proteção Wi-Fi & LAN", "Protección Wi-Fi y LAN", "WLAN- & LAN-Schutz", "Protection Wi-Fi et LAN", "Защита Wi-Fi и LAN", "Wi-Fi / LANの保護"),
    (lan_protect_desc, set_lan_protect_desc, "Prevents anyone on the same router from capturing your stream with packet analyzers such as Wireshark.", "Impede que qualquer pessoa no mesmo roteador capture seu stream com analisadores de pacotes como Wireshark.", "Impide que cualquier persona en el mismo router capture tu transmisión con analizadores de paquetes como Wireshark.", "Verhindert, dass jemand im selben Router deine Übertragung mit Paketanalysatoren wie Wireshark mitschneidet.", "Empêche toute personne sur le même routeur de capturer votre diffusion avec des analyseurs de paquets comme Wireshark.", "Не позволяет никому в той же сети перехватить вашу трансляцию анализаторами пакетов вроде Wireshark.", "同じルーターに接続している他人が、Wiresharkなどのパケット解析ツールで配信を傍受するのを防ぎます。"),
    (vault_title, set_vault_title, "Windows DPAPI Credential Vault", "Cofre de Credenciais Windows DPAPI", "Almacén de credenciales Windows DPAPI", "Windows-DPAPI-Anmeldedatentresor", "Coffre d'identifiants Windows DPAPI", "Хранилище учётных данных Windows DPAPI", "Windows DPAPI 資格情報ストア"),
    (protected, set_protected, "PROTECTED", "PROTEGIDO", "PROTEGIDO", "GESCHÜTZT", "PROTÉGÉ", "ЗАЩИЩЕНО", "保護済み"),
    (vault_desc, set_vault_desc, "Your Discord token is stored with native CryptProtectData encryption bound to your Windows user account.", "Seu token do Discord é armazenado com criptografia nativa CryptProtectData vinculada à sua conta de usuário do Windows.", "Tu token de Discord se guarda con cifrado nativo CryptProtectData vinculado a tu cuenta de usuario de Windows.", "Dein Discord-Token wird mit nativer CryptProtectData-Verschlüsselung gespeichert, die an dein Windows-Benutzerkonto gebunden ist.", "Votre token Discord est stocké avec le chiffrement natif CryptProtectData lié à votre compte utilisateur Windows.", "Ваш токен Discord хранится с нативным шифрованием CryptProtectData, привязанным к вашей учётной записи Windows.", "Discordトークンは、Windowsユーザーアカウントに紐づくネイティブのCryptProtectData暗号化で保存されます。"),
    (stealer_immunity, set_stealer_immunity, "Stealer Immunity", "Imunidade a Stealers", "Inmunidad frente a stealers", "Immunität gegen Stealer", "Immunité contre les stealers", "Защита от стилеров", "スティーラーへの耐性"),
    (stealer_immunity_desc, set_stealer_immunity_desc, "Unauthorized scripts and malware cannot steal your session because the cryptographic key lives in the Windows kernel.", "Scripts e malwares não autorizados não conseguem roubar sua sessão porque a chave criptográfica reside no kernel do Windows.", "Los scripts y el malware no autorizados no pueden robar tu sesión porque la clave criptográfica reside en el kernel de Windows.", "Nicht autorisierte Skripte und Malware können deine Sitzung nicht stehlen, da der kryptografische Schlüssel im Windows-Kernel liegt.", "Les scripts et logiciels malveillants non autorisés ne peuvent pas voler votre session, car la clé cryptographique réside dans le noyau Windows.", "Неавторизованные скрипты и вредоносные программы не могут украсть вашу сессию, так как криптографический ключ находится в ядре Windows.", "暗号鍵はWindowsカーネル内にあるため、不正なスクリプトやマルウェアがセッションを盗むことはできません。"),
    (dave_title, set_dave_title, "Official Discord Voice (DAVE Protocol)", "Voz Discord Oficial (Protocolo DAVE)", "Voz oficial de Discord (protocolo DAVE)", "Offizielle Discord-Sprache (DAVE-Protokoll)", "Voix officielle Discord (protocole DAVE)", "Официальный голос Discord (протокол DAVE)", "Discord公式音声 (DAVEプロトコル)"),
    (dave_desc, set_dave_desc, "End-to-end encrypted voice communication with Discord's official voice servers, with no intermediaries.", "Comunicação de voz criptografada ponta a ponta com os servidores de voz oficiais do Discord sem intermediários.", "Comunicación de voz cifrada de extremo a extremo con los servidores de voz oficiales de Discord, sin intermediarios.", "Ende-zu-Ende-verschlüsselte Sprachkommunikation mit den offiziellen Discord-Sprachservern, ohne Zwischenstellen.", "Communication vocale chiffrée de bout en bout avec les serveurs vocaux officiels de Discord, sans intermédiaires.", "Голосовая связь со сквозным шифрованием с официальными голосовыми серверами Discord, без посредников.", "Discord公式の音声サーバーとのエンドツーエンド暗号化音声通信です。仲介はありません。"),
    (zero_intermediaries, set_zero_intermediaries, "No telemetry", "Sem telemetria", "Sin telemetría", "Keine Telemetrie", "Aucune télémétrie", "Без телеметрии", "テレメトリなし"),
    (zero_intermediaries_desc, set_zero_intermediaries_desc, "Chat and calls go straight to Discord. Litecord's own server only puts screen-share participants in touch (the video goes directly between them, encrypted), and GitHub is checked for updates. No usage data is sent anywhere.", "Chat e chamadas vão direto para o Discord. O servidor do Litecord só põe em contato quem participa de uma transmissão (o vídeo vai direto entre vocês, criptografado), e o GitHub é consultado para atualizações. Nenhum dado de uso é enviado.", "El chat y las llamadas van directo a Discord. El servidor de Litecord solo pone en contacto a quienes participan en una transmisión (el video va directo entre ellos, cifrado), y se consulta GitHub para buscar actualizaciones. No se envían datos de uso.", "Chat und Anrufe gehen direkt zu Discord. Der Litecord-Server bringt nur die Teilnehmer einer Bildschirmübertragung zusammen (das Video läuft direkt und verschlüsselt zwischen ihnen), und GitHub wird nach Updates gefragt. Es werden keine Nutzungsdaten gesendet.", "Le chat et les appels vont directement à Discord. Le serveur de Litecord ne fait que mettre en relation les participants d'un partage d'écran (la vidéo passe directement entre eux, chiffrée), et GitHub est consulté pour les mises à jour. Aucune donnée d'utilisation n'est envoyée.", "Чат и звонки идут напрямую в Discord. Сервер Litecord только связывает участников трансляции экрана (видео идёт напрямую между ними, в зашифрованном виде), а GitHub проверяется на наличие обновлений. Данные об использовании никуда не отправляются.", "チャットと通話はDiscordに直接接続します。Litecordのサーバーは画面共有の参加者同士をつなぐだけで（映像は暗号化されて参加者間で直接やり取りされます）、更新の確認にはGitHubを使います。利用データはどこにも送信されません。"),
    (proxy_mode, set_proxy_mode, "Proxy Connection Mode", "Modo de Conexão Proxy", "Modo de conexión proxy", "Proxy-Verbindungsmodus", "Mode de connexion proxy", "Режим подключения через прокси", "プロキシ接続モード"),
    (proxy_address, set_proxy_address, "Proxy Address and Authentication", "Endereço e Autenticação do Proxy", "Dirección y autenticación del proxy", "Proxy-Adresse und Authentifizierung", "Adresse et authentification du proxy", "Адрес и аутентификация прокси", "プロキシのアドレスと認証"),
    (proxy_host, set_proxy_host, "Server / Host IP:", "Servidor / IP Host:", "Servidor / IP del host:", "Server / Host-IP:", "Serveur / IP de l'hôte :", "Сервер / IP хоста:", "サーバー / ホストIP:"),
    (proxy_port, set_proxy_port, "Port:", "Porta:", "Puerto:", "Port:", "Port :", "Порт:", "ポート:"),
    (proxy_user, set_proxy_user, "User (Optional):", "Usuário (Opcional):", "Usuario (opcional):", "Benutzer (optional):", "Utilisateur (facultatif) :", "Пользователь (необязательно):", "ユーザー (任意):"),
    (proxy_pass, set_proxy_pass, "Password (Optional):", "Senha (Opcional):", "Contraseña (opcional):", "Passwort (optional):", "Mot de passe (facultatif) :", "Пароль (необязательно):", "パスワード (任意):"),
    (save_apply, set_save_apply, "Save and Apply", "Salvar e Aplicar", "Guardar y aplicar", "Speichern und anwenden", "Enregistrer et appliquer", "Сохранить и применить", "保存して適用"),
    (bypass_blocks, set_bypass_blocks, "Bypass Blocks", "Burlar Bloqueios", "Eludir bloqueos", "Sperren umgehen", "Contourner les blocages", "Обход блокировок", "ブロックの回避"),
    (bypass_blocks_desc, set_bypass_blocks_desc, "Lets you get around firewalls and DNS blocks at schools or companies.", "Permite contornar firewalls e bloqueios de DNS em colégios ou empresas.", "Permite sortear cortafuegos y bloqueos de DNS en colegios o empresas.", "Ermöglicht das Umgehen von Firewalls und DNS-Sperren in Schulen oder Unternehmen.", "Permet de contourner les pare-feu et les blocages DNS dans les écoles ou les entreprises.", "Позволяет обходить брандмауэры и блокировки DNS в школах и на предприятиях.", "学校や会社のファイアウォールやDNSブロックを回避できます。"),
    (tor_desc, set_tor_desc, "Compatible with Tor's port 9050 and local SSH tunnels to anonymize your IP.", "Compatível com a porta 9050 do Tor e túneis SSH locais para anonimizar seu IP.", "Compatible con el puerto 9050 de Tor y túneles SSH locales para anonimizar tu IP.", "Kompatibel mit Tors Port 9050 und lokalen SSH-Tunneln, um deine IP zu anonymisieren.", "Compatible avec le port 9050 de Tor et les tunnels SSH locaux pour anonymiser votre IP.", "Совместим с портом 9050 Tor и локальными SSH-туннелями для анонимизации вашего IP.", "Torのポート9050やローカルのSSHトンネルに対応し、IPアドレスを匿名化できます。"),
    (debug_desc, set_debug_desc, "Point to 127.0.0.1:8080 to inspect REST API requests.", "Aponte para 127.0.0.1:8080 para inspecionar requisições da API REST.", "Apunta a 127.0.0.1:8080 para inspeccionar las peticiones a la API REST.", "Auf 127.0.0.1:8080 zeigen, um REST-API-Anfragen zu untersuchen.", "Pointez vers 127.0.0.1:8080 pour inspecter les requêtes de l'API REST.", "Укажите 127.0.0.1:8080, чтобы просматривать запросы к REST API.", "127.0.0.1:8080 を指定すると、REST APIのリクエストを検査できます。"),
    (gaming_perf, set_gaming_perf, "Gaming Performance", "Desempenho em Jogos", "Rendimiento en juegos", "Leistung in Spielen", "Performances en jeu", "Производительность в играх", "ゲームでのパフォーマンス"),
    (gaming_perf_desc, set_gaming_perf_desc, "With the proxy disabled, the impact is 0%. Voice and video keep direct UDP routes.", "Com proxy desativado, o impacto é 0%. Voz e vídeo mantêm rotas UDP diretas.", "Con el proxy desactivado, el impacto es del 0 %. La voz y el vídeo mantienen rutas UDP directas.", "Bei deaktiviertem Proxy beträgt die Auswirkung 0 %. Sprache und Video behalten direkte UDP-Routen.", "Avec le proxy désactivé, l'impact est de 0 %. La voix et la vidéo conservent des routes UDP directes.", "При отключённом прокси влияние составляет 0 %. Голос и видео сохраняют прямые UDP-маршруты.", "プロキシを無効にすると影響は0%です。音声と映像は直接のUDP経路を維持します。"),
    (release_channel, set_release_channel, "Official Release Channel", "Canal Oficial de Releases", "Canal oficial de versiones", "Offizieller Release-Kanal", "Canal officiel des versions", "Официальный канал релизов", "公式リリースチャンネル"),
    (release_channel_desc, set_release_channel_desc, "Updates are checked directly in the GitHub repository with domain signature validation.", "As atualizações são verificadas diretamente no repositório do GitHub com validação de assinatura de domínio.", "Las actualizaciones se comprueban directamente en el repositorio de GitHub con validación de firma del dominio.", "Updates werden direkt im GitHub-Repository mit Domain-Signaturprüfung abgerufen.", "Les mises à jour sont vérifiées directement dans le dépôt GitHub avec validation de la signature du domaine.", "Обновления проверяются напрямую в репозитории GitHub с проверкой подписи домена.", "アップデートはGitHubリポジトリで直接確認され、ドメイン署名の検証が行われます。"),
    (rust_perf, set_rust_perf, "Native Rust Performance", "Performance Nativa em Rust", "Rendimiento nativo en Rust", "Native Rust-Leistung", "Performances natives en Rust", "Нативная производительность на Rust", "Rustネイティブの高いパフォーマンス"),
    (rust_perf_desc, set_rust_perf_desc, "• CPU usage: < 0.1% when idle (0.0% in DeepSleep mode in the tray)\n• RAM: ~32 MB (versus over 700 MB for Electron)\n• Opus PLC audio pipeline with smart volume ducking", "• Uso de CPU: < 0.1% em idle (0.0% no modo DeepSleep na bandeja)\n• Memória RAM: ~32 MB (contra mais de 700 MB do Electron)\n• Pipeline de áudio Opus PLC com atenuação de volume inteligente", "• Uso de CPU: < 0,1 % en reposo (0,0 % en modo DeepSleep en la bandeja)\n• RAM: ~32 MB (frente a más de 700 MB de Electron)\n• Canal de audio Opus PLC con atenuación inteligente del volumen", "• CPU-Auslastung: < 0,1 % im Leerlauf (0,0 % im DeepSleep-Modus im Tray)\n• RAM: ~32 MB (gegenüber über 700 MB bei Electron)\n• Opus-PLC-Audiopipeline mit intelligenter Lautstärkeabsenkung", "• Utilisation CPU : < 0,1 % au repos (0,0 % en mode DeepSleep dans la barre d'état)\n• RAM : ~32 Mo (contre plus de 700 Mo pour Electron)\n• Pipeline audio Opus PLC avec atténuation intelligente du volume", "• Загрузка CPU: < 0,1 % в простое (0,0 % в режиме DeepSleep в трее)\n• RAM: ~32 МБ (против более 700 МБ у Electron)\n• Аудиоконвейер Opus PLC с умным приглушением громкости", "• CPU使用率: アイドル時 0.1%未満 (トレイのDeepSleepモードでは0.0%)\n• RAM: 約32MB (Electronの700MB超に対して)\n• スマートな音量ダッキング対応のOpus PLCオーディオパイプライン"),
    (deepsleep, set_deepsleep, "DeepSleep Mode", "Modo DeepSleep", "Modo DeepSleep", "DeepSleep-Modus", "Mode DeepSleep", "Режим DeepSleep", "DeepSleepモード"),
    (deepsleep_desc, set_deepsleep_desc, "When minimized to the tray, the interface freezes its rendering to guarantee 100% FPS for your games.", "Ao minimizar para a bandeja, a interface congela sua renderização para garantir 100% de FPS para seus jogos.", "Al minimizar a la bandeja, la interfaz congela su renderizado para garantizar el 100 % de FPS en tus juegos.", "Beim Minimieren in den Tray friert die Oberfläche ihr Rendering ein, um 100 % FPS für deine Spiele zu garantieren.", "Lorsqu'elle est réduite dans la barre d'état, l'interface fige son rendu pour garantir 100 % des FPS à vos jeux.", "При сворачивании в трей интерфейс замораживает отрисовку, чтобы гарантировать 100 % FPS для ваших игр.", "トレイに最小化すると、ゲームのFPSを100%確保するため、インターフェースは描画を停止します。"),
    (used_command, set_used_command, "used", "usou", "usó", "verwendete", "a utilisé", "использовал(а)", "使用:"),
    (settings_tab_video, set_settings_tab_video, "Video & Interface", "Vídeo e Interface", "Vídeo e interfaz", "Video & Oberfläche", "Vidéo et interface", "Видео и интерфейс", "ビデオとインターフェース"),
    (stop_stream, set_stop_stream, "Stop Streaming", "Parar Transmissão", "Detener transmisión", "Übertragung beenden", "Arrêter la diffusion", "Остановить трансляцию", "配信を停止"),
    (watching_stream, set_watching_stream, "Watching Stream", "Assistindo Transmissão", "Viendo transmisión", "Übertragung wird angesehen", "Visionnage de la diffusion", "Просмотр трансляции", "配信を視聴中"),
    (stream_word, set_stream_word, "Stream", "Transmissão", "Transmisión", "Übertragung", "Diffusion", "Трансляция", "配信"),
    (waiting_token, set_waiting_token, "Waiting for Access Token...", "Aguardando Token de Acesso...", "Esperando token de acceso...", "Warte auf Zugriffstoken...", "En attente du jeton d'accès...", "Ожидание токена доступа...", "アクセストークンを待機中..."),
    (not_connected, set_not_connected, "Not connected", "Não conectado", "No conectado", "Nicht verbunden", "Non connecté", "Не подключено", "未接続"),
    (select_channel, set_select_channel, "Select a channel", "Selecione um canal", "Selecciona un canal", "Wähle einen Kanal", "Sélectionnez un salon", "Выберите канал", "チャンネルを選択"),
    (update_downloading, set_update_downloading, "Downloading update...", "Baixando atualização...", "Descargando actualización...", "Update wird heruntergeladen...", "Téléchargement de la mise à jour...", "Загрузка обновления...", "アップデートをダウンロード中..."),
    (update_comparing, set_update_comparing, "Comparing local version with GitHub...", "Comparando versão local com o GitHub...", "Comparando la versión local con GitHub...", "Lokale Version wird mit GitHub verglichen...", "Comparaison de la version locale avec GitHub...", "Сравнение локальной версии с GitHub...", "ローカルのバージョンをGitHubと比較中..."),
    (proxy_saved, set_proxy_saved, "Proxy settings saved successfully!", "Configurações de proxy salvas com sucesso!", "¡Ajustes del proxy guardados correctamente!", "Proxy-Einstellungen erfolgreich gespeichert!", "Paramètres du proxy enregistrés avec succès !", "Настройки прокси успешно сохранены!", "プロキシ設定を保存しました！"),
    (proxy_testing, set_proxy_testing, "Testing connection through proxy...", "Testando conexão via proxy...", "Probando la conexión a través del proxy...", "Verbindung über Proxy wird getestet...", "Test de la connexion via le proxy...", "Проверка подключения через прокси...", "プロキシ経由の接続をテスト中..."),
    (qr_expired, set_qr_expired, "The QR Code session expired or was cancelled on the phone. Click Generate QR to try again.", "Sessão do QR Code expirou ou foi cancelada no celular. Clique em Gerar QR para tentar novamente.", "La sesión del código QR expiró o se canceló en el teléfono. Haz clic en Generar QR para intentarlo de nuevo.", "Die QR-Code-Sitzung ist abgelaufen oder wurde am Handy abgebrochen. Klicke auf „QR generieren“, um es erneut zu versuchen.", "La session du QR Code a expiré ou a été annulée sur le téléphone. Cliquez sur « Générer le QR » pour réessayer.", "Сессия QR-кода истекла или была отменена на телефоне. Нажмите «Создать QR», чтобы повторить попытку.", "QRコードのセッションが期限切れになったか、スマートフォンでキャンセルされました。「QRを生成」をクリックして再試行してください。"),
    (qr_captcha, set_qr_captcha, "Discord required a Captcha verification for the QR Code. Please paste your Token next to it to connect right away.", "O Discord exigiu verificação por Captcha para o QR Code. Por favor, cole seu Token ao lado para conectar imediatamente.", "Discord exigió una verificación Captcha para el código QR. Pega tu token junto a este para conectarte de inmediato.", "Discord hat für den QR-Code eine Captcha-Verifizierung verlangt. Füge daneben deinen Token ein, um dich sofort zu verbinden.", "Discord a exigé une vérification Captcha pour le QR Code. Collez votre token à côté pour vous connecter immédiatement.", "Discord потребовал проверку Captcha для QR-кода. Вставьте рядом свой токен, чтобы подключиться сразу.", "DiscordがQRコードにCaptcha認証を要求しました。横にトークンを貼り付けると、すぐに接続できます。"),
    (signed_out, set_signed_out, "You signed out. Enter your token or scan the QR Code.", "Você saiu da conta. Insira seu token ou escaneie o QR Code.", "Cerraste sesión. Introduce tu token o escanea el código QR.", "Du hast dich abgemeldet. Gib deinen Token ein oder scanne den QR-Code.", "Vous vous êtes déconnecté. Saisissez votre token ou scannez le QR Code.", "Вы вышли из аккаунта. Введите токен или отсканируйте QR-код.", "ログアウトしました。トークンを入力するか、QRコードをスキャンしてください。"),
    (proxy_failed, set_proxy_failed, "Connection failed via Proxy ({})", "Falha de conexão via Proxy ({})", "Error de conexión mediante proxy ({})", "Verbindung über Proxy fehlgeschlagen ({})", "Échec de la connexion via le proxy ({})", "Не удалось подключиться через прокси ({})", "プロキシ経由の接続に失敗しました ({})"),
    (auto_connecting, set_auto_connecting, "Auto-connecting to Gateway v9...", "Autoconectando à Gateway v9...", "Conectando automáticamente al Gateway v9...", "Automatische Verbindung zum Gateway v9...", "Connexion automatique à la Gateway v9...", "Автоподключение к Gateway v9...", "Gateway v9 に自動接続中..."),
    (perms_unavailable, set_perms_unavailable, "Permissions unavailable", "Permissões indisponíveis", "Permisos no disponibles", "Berechtigungen nicht verfügbar", "Autorisations indisponibles", "Права недоступны", "権限を利用できません"),
    (perms_failed, set_perms_failed, "Failed to check permissions. Select the server again to try.", "Falha ao verificar permissões. Selecione o servidor novamente para tentar.", "No se pudieron comprobar los permisos. Selecciona el servidor de nuevo para reintentar.", "Berechtigungen konnten nicht geprüft werden. Wähle den Server erneut aus, um es nochmals zu versuchen.", "Impossible de vérifier les autorisations. Sélectionnez à nouveau le serveur pour réessayer.", "Не удалось проверить права. Выберите сервер ещё раз, чтобы повторить попытку.", "権限を確認できませんでした。もう一度サーバーを選択して再試行してください。"),
    (no_text_channel, set_no_text_channel, "No accessible text channel", "Nenhum canal de texto acessível", "Ningún canal de texto accesible", "Kein zugänglicher Textkanal", "Aucun salon textuel accessible", "Нет доступных текстовых каналов", "アクセス可能なテキストチャンネルがありません"),
    (press_enter_run, set_press_enter_run, "Press Enter to run ", "Pressione Enter para executar ", "Pulsa Intro para ejecutar ", "Drücke die Eingabetaste, um auszuführen: ", "Appuyez sur Entrée pour exécuter ", "Нажмите Enter, чтобы выполнить ", "Enterキーで実行: "),
    (type_value_for, set_type_value_for, "Type the value for ", "Digite o valor para ", "Escribe el valor para ", "Wert eingeben für ", "Saisissez la valeur pour ", "Введите значение для ", "値を入力: "),
    (press_enter_or_send, set_press_enter_or_send, "Press Enter or Send to run ", "Pressione Enter ou Enviar para executar ", "Pulsa Intro o Enviar para ejecutar ", "Eingabetaste oder Senden drücken, um auszuführen: ", "Appuyez sur Entrée ou Envoyer pour exécuter ", "Нажмите Enter или «Отправить», чтобы выполнить ", "Enterキーまたは送信で実行: "),
    (now_label, set_now_label, "Now", "Agora", "Ahora", "Jetzt", "Maintenant", "Сейчас", "今"),
    (welcome_msg, set_welcome_msg, "Welcome to Litecord! Your servers will load in the sidebar on the left.", "Bem-vindo ao Litecord! Seus servidores carregarão na barra lateral à esquerda.", "¡Bienvenido a Litecord! Tus servidores se cargarán en la barra lateral de la izquierda.", "Willkommen bei Litecord! Deine Server werden in der Seitenleiste links geladen.", "Bienvenue sur Litecord ! Vos serveurs se chargeront dans la barre latérale de gauche.", "Добро пожаловать в Litecord! Ваши серверы загрузятся на боковой панели слева.", "Litecordへようこそ！サーバーは左のサイドバーに読み込まれます。"),
    (label_muted, set_label_muted, "Muted", "Mudo", "Silenciado", "Stummgeschaltet", "Muet", "Микрофон выключен", "ミュート中"),
    (label_transmitting, set_label_transmitting, "Transmitting...", "Transmitindo...", "Transmitiendo...", "Überträgt...", "Transmission...", "Передача...", "送信中..."),
    (label_mic_active, set_label_mic_active, "Mic active", "Microfone ativo", "Micrófono activo", "Mikrofon aktiv", "Micro actif", "Микрофон включён", "マイク有効"),
    (label_muted_caps, set_label_muted_caps, "MUTED", "MUDO", "SILENCIADO", "STUMM", "MUET", "БЕЗ ЗВУКА", "ミュート"),
    (update_prompt, set_update_prompt, "A new version of Litecord with improvements and fixes is ready to install. Update now?", "Uma nova versão do Litecord com melhorias e correções está pronta para instalação. Deseja atualizar agora?", "Una nueva versión de Litecord con mejoras y correcciones está lista para instalarse. ¿Actualizar ahora?", "Eine neue Version von Litecord mit Verbesserungen und Fehlerbehebungen ist installationsbereit. Jetzt aktualisieren?", "Une nouvelle version de Litecord avec des améliorations et des correctifs est prête à être installée. Mettre à jour maintenant ?", "Новая версия Litecord с улучшениями и исправлениями готова к установке. Обновить сейчас?", "改善と修正を含むLitecordの新バージョンをインストールできます。今すぐ更新しますか？"),
    (screen_window, set_screen_window, "Screen / Window", "Tela / Janela", "Pantalla / Ventana", "Bildschirm / Fenster", "Écran / Fenêtre", "Экран / Окно", "画面 / ウィンドウ"),
    (system_portal, set_system_portal, "Native system portal", "Portal nativo do sistema", "Portal nativo del sistema", "Natives System-Portal", "Portail natif du système", "Системный портал", "システムネイティブのポータル"),
    (app_music, set_app_music, "Music", "Música", "Música", "Musik", "Musique", "Музыка", "音楽"),
    (app_moderation, set_app_moderation, "Moderation", "Moderação", "Moderación", "Moderation", "Modération", "Модерация", "モデレーション"),
    (app_utility, set_app_utility, "Utility", "Utilidade", "Utilidad", "Dienstprogramm", "Utilitaire", "Утилиты", "ユーティリティ"),
    (cmd_play_desc, set_cmd_play_desc, "Play music, audio or a playlist (YouTube/Spotify/SoundCloud)", "Tocar música, áudio ou playlist (YouTube/Spotify/SoundCloud)", "Reproduce música, audio o una lista (YouTube/Spotify/SoundCloud)", "Musik, Audio oder Playlist abspielen (YouTube/Spotify/SoundCloud)", "Lire de la musique, de l'audio ou une playlist (YouTube/Spotify/SoundCloud)", "Воспроизвести музыку, аудио или плейлист (YouTube/Spotify/SoundCloud)", "音楽、音声、プレイリストを再生 (YouTube/Spotify/SoundCloud)"),
    (cmd_play_usage, set_cmd_play_usage, "/play <name or link>", "/play <nome ou link>", "/play <nombre o enlace>", "/play <Name oder Link>", "/play <nom ou lien>", "/play <название или ссылка>", "/play <名前またはリンク>"),
    (cmd_play_param, set_cmd_play_param, "Song name or YouTube/Spotify/SoundCloud link", "Nome da música ou link do YouTube/Spotify/SoundCloud", "Nombre de la canción o enlace de YouTube/Spotify/SoundCloud", "Songname oder YouTube/Spotify/SoundCloud-Link", "Nom du morceau ou lien YouTube/Spotify/SoundCloud", "Название трека или ссылка YouTube/Spotify/SoundCloud", "曲名またはYouTube/Spotify/SoundCloudのリンク"),
    (cmd_skip_desc, set_cmd_skip_desc, "Skip to the next track in the queue", "Pular para a próxima faixa da fila", "Saltar a la siguiente pista de la cola", "Zum nächsten Titel der Warteschlange springen", "Passer au morceau suivant de la file", "Перейти к следующему треку в очереди", "キューの次のトラックへスキップ"),
    (cmd_stop_desc, set_cmd_stop_desc, "Stop the bot's playback and clear the queue", "Parar reprodução do bot e limpar a fila", "Detener la reproducción del bot y vaciar la cola", "Wiedergabe des Bots stoppen und Warteschlange leeren", "Arrêter la lecture du bot et vider la file", "Остановить воспроизведение бота и очистить очередь", "ボットの再生を停止し、キューを空にする"),
    (cmd_pause_desc, set_cmd_pause_desc, "Pause the song that is playing", "Pausar a música em reprodução", "Pausar la canción en reproducción", "Den laufenden Titel pausieren", "Mettre en pause le morceau en cours", "Поставить текущий трек на паузу", "再生中の曲を一時停止"),
    (cmd_resume_desc, set_cmd_resume_desc, "Resume the paused playback", "Retomar a reprodução pausada", "Reanudar la reproducción en pausa", "Pausierte Wiedergabe fortsetzen", "Reprendre la lecture en pause", "Возобновить воспроизведение", "一時停止中の再生を再開"),
    (cmd_queue_desc, set_cmd_queue_desc, "Show the list of songs in the queue", "Visualizar a lista de músicas na fila", "Ver la lista de canciones en la cola", "Liste der Titel in der Warteschlange anzeigen", "Afficher la liste des morceaux dans la file", "Показать список треков в очереди", "キュー内の曲の一覧を表示"),
    (cmd_nowplaying_desc, set_cmd_nowplaying_desc, "Show details of the song playing right now", "Exibir detalhes da música que está tocando agora", "Mostrar detalles de la canción que suena ahora", "Details zum aktuell gespielten Titel anzeigen", "Afficher les détails du morceau en cours de lecture", "Показать сведения о текущем треке", "再生中の曲の詳細を表示"),
    (cmd_volume_desc, set_cmd_volume_desc, "Adjust the bot's playback volume", "Ajustar o volume de reprodução do bot", "Ajustar el volumen de reproducción del bot", "Wiedergabelautstärke des Bots anpassen", "Régler le volume de lecture du bot", "Настроить громкость воспроизведения бота", "ボットの再生音量を調整"),
    (cmd_volume_param, set_cmd_volume_param, "Volume from 1 to 100", "Volume de 1 a 100", "Volumen de 1 a 100", "Lautstärke von 1 bis 100", "Volume de 1 à 100", "Громкость от 1 до 100", "音量 (1〜100)"),
    (cmd_loop_desc, set_cmd_loop_desc, "Enable or disable repeat for the song or queue", "Ativar ou desativar repetição da música ou fila", "Activar o desactivar la repetición de la canción o la cola", "Wiederholung des Titels oder der Warteschlange ein- oder ausschalten", "Activer ou désactiver la répétition du morceau ou de la file", "Включить или выключить повтор трека или очереди", "曲またはキューのリピートをオン/オフ"),
    (cmd_loop_param, set_cmd_loop_param, "Mode: track, queue or off", "Modo: track, queue ou off", "Modo: track, queue u off", "Modus: track, queue oder off", "Mode : track, queue ou off", "Режим: track, queue или off", "モード: track、queue、off"),
    (cmd_shuffle_desc, set_cmd_shuffle_desc, "Shuffle the song queue", "Embaralhar a fila de músicas", "Mezclar la cola de canciones", "Warteschlange mischen", "Mélanger la file de morceaux", "Перемешать очередь треков", "曲のキューをシャッフル"),
    (cmd_lyrics_desc, set_cmd_lyrics_desc, "Search for and show the lyrics of the current song", "Buscar e exibir a letra da música atual", "Buscar y mostrar la letra de la canción actual", "Songtext des aktuellen Titels suchen und anzeigen", "Rechercher et afficher les paroles du morceau actuel", "Найти и показать текст текущей песни", "現在の曲の歌詞を検索して表示"),
    (cmd_lyrics_param, set_cmd_lyrics_param, "Song name (optional)", "Nome da música (opcional)", "Nombre de la canción (opcional)", "Songname (optional)", "Nom du morceau (facultatif)", "Название песни (необязательно)", "曲名 (任意)"),
    (cmd_247_desc, set_cmd_247_desc, "Keep the bot connected 24/7 in the voice channel", "Manter o bot 24/7 conectado no canal de voz", "Mantener el bot conectado 24/7 en el canal de voz", "Den Bot rund um die Uhr im Sprachkanal verbunden halten", "Garder le bot connecté 24h/24 et 7j/7 dans le salon vocal", "Держать бота подключённым к голосовому каналу 24/7", "ボットをボイスチャンネルに24時間365日接続したままにする"),
    (cmd_help_desc, set_cmd_help_desc, "Show the bot's help menu and commands", "Exibir o menu de ajuda e comandos do bot", "Mostrar el menú de ayuda y los comandos del bot", "Hilfemenü und Befehle des Bots anzeigen", "Afficher le menu d'aide et les commandes du bot", "Показать меню справки и команды бота", "ボットのヘルプメニューとコマンドを表示"),
    (cmd_ping_desc, set_cmd_ping_desc, "Check the latency and response time", "Verificar a latência e tempo de resposta", "Comprobar la latencia y el tiempo de respuesta", "Latenz und Antwortzeit prüfen", "Vérifier la latence et le temps de réponse", "Проверить задержку и время отклика", "レイテンシと応答時間を確認"),
    (cmd_clear_desc, set_cmd_clear_desc, "Clear recent messages in the channel", "Limpar mensagens recentes do canal", "Borrar los mensajes recientes del canal", "Letzte Nachrichten im Kanal löschen", "Supprimer les messages récents du salon", "Удалить недавние сообщения в канале", "チャンネルの最近のメッセージを削除"),
    (cmd_clear_usage, set_cmd_clear_usage, "/clear <amount>", "/clear <quantidade>", "/clear <cantidad>", "/clear <Anzahl>", "/clear <nombre>", "/clear <количество>", "/clear <件数>"),
    (cmd_clear_param, set_cmd_clear_param, "Number of messages to delete", "Quantidade de mensagens a apagar", "Cantidad de mensajes a borrar", "Anzahl der zu löschenden Nachrichten", "Nombre de messages à supprimer", "Количество сообщений для удаления", "削除するメッセージ数"),
    (cmd_avatar_desc, set_cmd_avatar_desc, "Show a user's enlarged avatar", "Exibir o avatar ampliado de um usuário", "Mostrar el avatar ampliado de un usuario", "Vergrößerten Avatar eines Nutzers anzeigen", "Afficher l'avatar agrandi d'un utilisateur", "Показать увеличенный аватар пользователя", "ユーザーのアバターを拡大表示"),
    (cmd_avatar_usage, set_cmd_avatar_usage, "/avatar [@user]", "/avatar [@usuario]", "/avatar [@usuario]", "/avatar [@Nutzer]", "/avatar [@utilisateur]", "/avatar [@пользователь]", "/avatar [@ユーザー]"),
    (cmd_user_param, set_cmd_user_param, "Discord user", "Usuário do Discord", "Usuario de Discord", "Discord-Nutzer", "Utilisateur Discord", "Пользователь Discord", "Discordユーザー"),
    (cmd_userinfo_desc, set_cmd_userinfo_desc, "View detailed information about a user's account", "Ver informações detalhadas da conta de um usuário", "Ver información detallada de la cuenta de un usuario", "Detaillierte Kontoinformationen eines Nutzers anzeigen", "Afficher les informations détaillées du compte d'un utilisateur", "Показать подробные сведения об аккаунте пользователя", "ユーザーのアカウントの詳細情報を表示"),
    (cmd_userinfo_usage, set_cmd_userinfo_usage, "/userinfo [@user]", "/userinfo [@usuario]", "/userinfo [@usuario]", "/userinfo [@Nutzer]", "/userinfo [@utilisateur]", "/userinfo [@пользователь]", "/userinfo [@ユーザー]"),
    (cmd_serverinfo_desc, set_cmd_serverinfo_desc, "View statistics and information about the current server", "Ver estatísticas e informações do servidor atual", "Ver estadísticas e información del servidor actual", "Statistiken und Informationen zum aktuellen Server anzeigen", "Afficher les statistiques et informations du serveur actuel", "Показать статистику и сведения о текущем сервере", "現在のサーバーの統計と情報を表示"),
    (mic_state_active, set_mic_state_active, "ACTIVE", "ATIVO", "ACTIVO", "AKTIV", "ACTIF", "АКТИВНО", "有効"),
    (mic_state_silent, set_mic_state_silent, "SILENCE", "SILÊNCIO", "SILENCIO", "STILLE", "SILENCE", "ТИШИНА", "無音"),
    (keybind_none, set_keybind_none, "None / Disabled", "Nenhum / Desativado", "Ninguno / Desactivado", "Keine / Deaktiviert", "Aucun / Désactivé", "Нет / Отключено", "なし / 無効"),
    (recording, set_recording, "Recording...", "Gravando...", "Grabando...", "Aufnahme...", "Enregistrement...", "Запись...", "記録中..."),
    (testing, set_testing, "Testing...", "Testando...", "Probando...", "Teste...", "Test en cours...", "Проверка...", "テスト中..."),
    (test_proxy, set_test_proxy, "Test Proxy", "Testar Proxy", "Probar proxy", "Proxy testen", "Tester le proxy", "Проверить прокси", "プロキシをテスト"),
    (checking, set_checking, "Checking...", "Verificando...", "Comprobando...", "Prüfe...", "Vérification...", "Проверка...", "確認中..."),
    (check_updates, set_check_updates, "Check for Updates", "Verificar Atualizações", "Buscar actualizaciones", "Nach Updates suchen", "Rechercher des mises à jour", "Проверить обновления", "アップデートを確認"),
    (loading_older, set_loading_older, "Loading older messages...", "Carregando mensagens anteriores...", "Cargando mensajes anteriores...", "Ältere Nachrichten werden geladen...", "Chargement des messages précédents...", "Загрузка предыдущих сообщений...", "以前のメッセージを読み込み中..."),
    (load_older, set_load_older, "Load older messages", "Carregar mensagens mais antigas", "Cargar mensajes más antiguos", "Ältere Nachrichten laden", "Charger les messages plus anciens", "Загрузить более старые сообщения", "古いメッセージを読み込む"),
    (downloading, set_downloading, "Downloading...", "Baixando...", "Descargando...", "Wird heruntergeladen...", "Téléchargement...", "Загрузка...", "ダウンロード中..."),
    (download_image_size, set_download_image_size, "Download Image (", "Baixar Imagem (", "Descargar imagen (", "Bild herunterladen (", "Télécharger l'image (", "Скачать изображение (", "画像をダウンロード ("),
    (download_image, set_download_image, "Download Image", "Baixar Imagem", "Descargar imagen", "Bild herunterladen", "Télécharger l'image", "Скачать изображение", "画像をダウンロード"),
    (attachment_label, set_attachment_label, "Attachment", "Anexo", "Adjunto", "Anhang", "Pièce jointe", "Вложение", "添付ファイル"),
    // Discord relative timestamps (<t:...:R>); {} is "5 min", "3 h" or "2 d".
    (time_in, set_time_in, "in {}", "em {}", "en {}", "in {}", "dans {}", "через {}", "{}後"),
    (time_ago, set_time_ago, "{} ago", "há {}", "hace {}", "vor {}", "il y a {}", "{} назад", "{}前"),
    (time_soon, set_time_soon, "in a moment", "em instantes", "en un momento", "gleich", "dans un instant", "сейчас", "まもなく"),
    (time_just_now, set_time_just_now, "just now", "há instantes", "hace un momento", "gerade eben", "à l'instant", "только что", "たった今"),
// Settings window (redesign).
    (settings_group_calls, set_settings_group_calls, "CALLS", "CHAMADAS", "LLAMADAS", "ANRUFE", "APPELS", "ЗВОНКИ", "通話"),
    (settings_group_app, set_settings_group_app, "APP", "APLICATIVO", "APLICACIÓN", "APP", "APPLICATION", "ПРИЛОЖЕНИЕ", "アプリ"),
    (settings_tab_appearance, set_settings_tab_appearance, "Appearance and language", "Aparência e idioma", "Apariencia e idioma", "Darstellung und Sprache", "Apparence et langue", "Внешний вид и язык", "外観と言語"),
    (settings_tab_about, set_settings_tab_about, "Privacy and about", "Privacidade e Sobre", "Privacidad y acerca de", "Datenschutz und Info", "Confidentialité et à propos", "Конфиденциальность и о программе", "プライバシーと情報"),
    (settings_advanced, set_settings_advanced, "Advanced", "Avançado", "Avanzado", "Erweitert", "Avancé", "Дополнительно", "詳細設定"),
    (settings_advanced_desc, set_settings_advanced_desc, "Change these only if the stream freezes or goes black. Automatic tries each option and falls back to the next one.", "Só mude se a transmissão travar ou ficar preta. O automático testa cada opção e passa para a seguinte se uma falhar.", "Cámbialo solo si la transmisión se congela o se queda en negro. Automático prueba cada opción y pasa a la siguiente si una falla.", "Nur ändern, wenn die Übertragung hängt oder schwarz bleibt. Automatisch probiert jede Option und wechselt bei einem Fehler zur nächsten.", "À changer seulement si le partage se fige ou reste noir. Automatique essaie chaque option et passe à la suivante en cas d’échec.", "Меняйте, только если трансляция зависает или остаётся чёрной. Автоматический режим пробует каждый вариант и переходит к следующему при ошибке.", "配信が止まる・真っ黒になる場合のみ変更してください。自動では各方式を試し、失敗すると次に切り替えます。"),
    (settings_privacy_title, set_settings_privacy_title, "How your data is protected", "Como seus dados são protegidos", "Cómo se protegen tus datos", "Wie deine Daten geschützt werden", "Comment vos données sont protégées", "Как защищены ваши данные", "データの保護について"),
    // Server invite cards; {} is a number already grouped for the language.
    (invite_join, set_invite_join, "Join", "Entrar", "Unirse", "Beitreten", "Rejoindre", "Вступить", "参加"),
    (invite_loading, set_invite_loading, "Loading invite…", "Carregando convite…", "Cargando invitación…", "Einladung wird geladen…", "Chargement de l’invitation…", "Загрузка приглашения…", "招待を読み込み中…"),
    (invite_online, set_invite_online, "{} online", "{} online", "{} en línea", "{} online", "{} en ligne", "{} в сети", "{}人がオンライン"),
    (invite_members, set_invite_members, "{} members", "{} membros", "{} miembros", "{} Mitglieder", "{} membres", "{} участников", "{}人のメンバー"),
    (event_view, set_event_view, "View event", "Ver evento", "Ver evento", "Event ansehen", "Voir l’événement", "Открыть событие", "イベントを見る"),
    (event_live, set_event_live, "Happening now", "Acontecendo agora", "En curso", "Läuft gerade", "En cours", "Идёт сейчас", "開催中"),
    (event_interested, set_event_interested, "{} interested", "{} interessados", "{} interesados", "{} interessiert", "{} intéressés", "{} заинтересованы", "{}人が興味あり"),
    (login_title, set_login_title, "Sign in to Litecord", "Entrar no Litecord", "Iniciar sesión en Litecord", "Bei Litecord anmelden", "Connexion à Litecord", "Вход в Litecord", "Litecord にログイン"),
    (login_desc, set_login_desc, "Enter your Discord User Token for direct connection to Gateway v9.\n(Estimated RAM usage ~15MB)", "Insira seu Discord User Token para conexão direta à Gateway v9.\n(Consumo de RAM estimado em ~15MB)", "Introduce tu Discord User Token para conexión directa a Gateway v9.\n(Consumo de RAM estimado en ~15MB)", "Gib deinen Discord User Token für direkte Gateway v9 Verbindung ein.\n(Geschätzter RAM-Verbrauch ~15MB)", "Entrez votre Discord User Token pour connexion directe à Gateway v9.\n(RAM estimée ~15Mo)", "Введите Discord User Token для подключения к Gateway v9.\n(Потребление ОЗУ ~15МБ)", "Gateway v9 に接続するために Discord ユーザートークンを入力してください。\n(推定RAM使用量: 約15MB)"),
    (login_placeholder, set_login_placeholder, "Paste your Discord User Token here...", "Cole seu Discord User Token aqui...", "Pega tu Discord User Token aquí...", "Discord User Token hier einfügen...", "Collez votre token Discord ici...", "Вставьте токен Discord сюда...", "Discord トークンをここに貼り付け..."),
    (login_btn_connect, set_login_btn_connect, "Connect to Gateway", "Conectar à Gateway", "Conectar a la Gateway", "Mit Gateway verbinden", "Se connecter à la Gateway", "Подключиться к Gateway", "Gateway に接続"),
    (login_btn_detect, set_login_btn_detect, "Detect Discord Token", "Detectar Token do Discord", "Detectar Token de Discord", "Discord Token erkennen", "Détecter le Token Discord", "Найти токен Discord", "Discord トークンを自動検出"),
    (server_channels, set_server_channels, "SERVER CHANNELS", "CANAIS DO SERVIDOR", "CANALES DEL SERVIDOR", "SERVERKANÄLE", "SALONS DU SERVEUR", "КАНАЛЫ СЕРВЕРА", "サーバーチャンネル"),
    (leave, set_leave, "Leave", "Sair", "Salir", "Verlassen", "Quitter", "Выйти", "退出"),
    (view_text_chat, set_view_text_chat, "View Text Chat", "Ver Chat de Texto", "Ver Chat de Texto", "Textchat anzeigen", "Voir le Salon Textuel", "Текстовый чат", "テキストチャットを表示"),
    (leave_call, set_leave_call, "Disconnect Call", "Sair da Call", "Desconectar Llamada", "Anruf beenden", "Déconnecter l'Appel", "Покинуть звонок", "通話を終了"),
    (voice_participants_title, set_voice_participants_title, "VOICE ROOM PARTICIPANTS", "PARTICIPANTES NA SALA DE ÁUDIO", "PARTICIPANTES EN LA SALA DE VOZ", "TEILNEHMER IM SPRACHRAUM", "PARTICIPANTS DANS LE SALON VOCAL", "УЧАСТНИКИ В ГОЛОСОВОМ КАНАЛЕ", "ボイスルームの参加者"),
    (badge_you, set_badge_you, "YOU", "VOCÊ", "TÚ", "DU", "VOUS", "ВЫ", "あなた"),
    (view_voice_room, set_view_voice_room, "View Voice Room", "Ver Sala de Voz", "Ver Sala de Voz", "Sprachraum anzeigen", "Voir le Salon Vocal", "Голосовой канал", "ボイスルームを表示"),
    (replying_to, set_replying_to, "Replying to @", "Respondendo a @", "Respondiendo a @", "Antwort auf @", "En réponse à @", "В ответ @", "返信先: @"),
    (chat_placeholder_prefix, set_chat_placeholder_prefix, "Message ", "Conversar em ", "Enviar mensaje a ", "Nachricht an ", "Envoyer un message dans ", "Написать в ", "メッセージを送信: "),
    (send, set_send, "Send", "Enviar", "Enviar", "Senden", "Envoyer", "Отправить", "送信"),
    (settings_language_label, set_settings_language_label, "LANGUAGE / IDIOMA", "IDIOMA / LANGUAGE", "IDIOMA / LANGUAGE", "SPRACHE / LANGUAGE", "LANGUE / LANGUAGE", "ЯЗЫК / LANGUAGE", "言語 / LANGUAGE"),
    (logout_title, set_logout_title, "Sign Out", "Sair da Conta", "Cerrar Sesión", "Abmelden", "Se Déconnecter", "Выйти из аккаунта", "ログアウト"),
    (logout_confirm_prefix, set_logout_confirm_prefix, "Do you want to log out from ", "Deseja desconectar de ", "¿Deseas desconectarte de ", "Möchtest du dich abmelden von ", "Voulez-vous vous déconnecter de ", "Вы действительно хотите выйти из ", "本当にログアウトしますか: "),
    (confirm_logout, set_confirm_logout, "Sign Out", "Sair", "Cerrar Sesión", "Abmelden", "Déconnexion", "Выйти", "ログアウト"),
    (voice_connecting, set_voice_connecting, "Connecting...", "Conectando...", "Conectando...", "Verbinden...", "Connexion...", "Подключение...", "接続中..."),
    (voice_connecting_title, set_voice_connecting_title, "Connecting to Voice Channel...", "Conectando à Sala de Voz...", "Conectando a la Sala de Voz...", "Verbindung zum Sprachkanal...", "Connexion au Salon Vocal...", "Подключение к голосовому каналу...", "ボイスチャンネルに接続中..."),
    (voice_connecting_desc, set_voice_connecting_desc, "Authenticating with Voice Gateway v9 and negotiating DAVE E2EE encryption...", "Autenticando na Voice Gateway v9 e negociando criptografia ponta-a-ponta (DAVE)...", "Autenticando en Voice Gateway v9 y negociando cifrado E2EE (DAVE)...", "Authentifizierung mit Voice Gateway v9 und Aushandlung der DAVE E2EE-Verschlüsselung...", "Authentification auprès de la Voice Gateway v9 et négociation du chiffrement DAVE E2EE...", "Аутентификация в Voice Gateway v9 и согласование E2EE шифрования (Дэйв)...", "Voice Gateway v9 に認証し、DAVE E2EE 暗号化を確立しています..."),
    (voice_connected, set_voice_connected, "Voice Connected", "Voz Conectada", "Voz Conectada", "Sprache verbunden", "Vocal Connecté", "Голос подключен", "音声接続完了"),
    (qr_title, set_qr_title, "Log in with QR Code", "Entrar com QR Code", "Iniciar sesión con código QR", "Mit QR-Code anmelden", "Se connecter avec un code QR", "Вход с помощью QR-кода", "QR コードでログイン"),
    (qr_desc, set_qr_desc, "Scan this with the Discord mobile app to log in instantly.", "Escaneie pelo aplicativo móvel do Discord para entrar instantaneamente.", "Escanéalo con la app móvil de Discord para entrar al instante.", "Mit der mobilen Discord-App scannen, um dich sofort anzumelden.", "Scannez avec l'application mobile Discord pour vous connecter instantanément.", "Сканируйте в мобильном приложении Discord для мгновенного входа.", "Discord モバイルアプリでスキャンしてすぐにログインできます。"),
    (qr_confirm, set_qr_confirm, "Approve login on your phone!", "Confirme o login no seu celular!", "¡Confirma el acceso en tu móvil!", "Bestätige die Anmeldung auf deinem Handy!", "Confirmez la connexion sur votre téléphone !", "Подтвердите вход на телефоне!", "スマホでログインを承認してください！"),
    (settings_title, set_settings_title, "Voice & Audio Settings", "Configurações de Voz e Áudio", "Ajustes de Voz y Audio", "Sprach- und Audioeinstellungen", "Paramètres Vocaux et Audio", "Настройки звука и голоса", "音声とオーディオ設定"),
    (settings_tab_voice, set_settings_tab_voice, "Voice & Audio", "Voz & Áudio", "Voz y Audio", "Sprach & Audio", "Vocal & Audio", "Голос и Звук", "音声 & オーディオ"),
    (settings_tab_language, set_settings_tab_language, "Language", "Idioma", "Idioma", "Sprache", "Langue", "Язык", "言語"),
    (settings_tab_keybinds, set_settings_tab_keybinds, "Keybinds", "Atalhos", "Atajos", "Tastenbelegung", "Raccourcis", "Горячие клавиши", "キー割り当て"),
    (settings_tab_security, set_settings_tab_security, "Security & E2EE", "Segurança & E2EE", "Seguridad y E2EE", "Sicherheit & E2EE", "Sécurité & E2EE", "Безопасность и E2EE", "セキュリティ & E2EE"),
    (settings_tab_updates, set_settings_tab_updates, "Updates & About", "Atualizações & Sobre", "Actualizaciones y Sobre", "Updates & Über", "Mises à jour & À propos", "Обновления и О программе", "アップデート & 詳細"),
    (settings_tab_network_proxy, set_settings_tab_network_proxy, "Network & Proxy", "Rede & Proxy", "Red y Proxy", "Netzwerk & Proxy", "Réseau & Proxy", "Сеть и Прокси", "ネットワーク＆プロキシ"),
    (settings_input_device, set_settings_input_device, "INPUT DEVICE (MICROPHONE)", "DISPOSITIVO DE ENTRADA (MICROFONE)", "DISPOSITIVO DE ENTRADA (MICRÓFONO)", "EINGABEGERÄT (MIKROFON)", "PÉRIPHÉRIQUE D'ENTRÉE (MICROPHONE)", "УСТРОЙСТВО ВВОДА (МИКРОФОН)", "入力デバイス (マイク)"),
    (settings_output_device, set_settings_output_device, "OUTPUT DEVICE (SPEAKERS / HEADPHONES)", "DISPOSITIVO DE SAÍDA (ALTO-FALANTES / FONE DE OUVIDO)", "DISPOSITIVO DE SALIDA (ALTAVOCES / AURICULARES)", "AUSGABEGERÄT (LAUTSPRECHER / KOPFHÖRER)", "PÉRIPHÉRIQUE DE SORTIE (CASQUE / ENCEINTES)", "УСТРОЙСТВО ВЫВОДА (ДИНАМИКИ / НАУШНИКИ)", "出力デバイス (スピーカー / ヘッドホン)"),
    (settings_threshold, set_settings_threshold, "VOICE DETECTION SENSITIVITY (THRESHOLD)", "SENSIBILIDADE DE DETECÇÃO DE VOZ (THRESHOLD)", "SENSIBILIDAD DE DETECCIÓN DE VOZ (THRESHOLD)", "SPRACHAKTIVIERUNG-EMPFINDLICHKEIT (THRESHOLD)", "SENSIBILITÉ DE DÉTECTION VOCALE (THRESHOLD)", "ЧУВСТВИТЕЛЬНОСТЬ ГОЛОСА (THRESHOLD)", "音声検出感度 (THRESHOLD)"),
    (settings_mic_level, set_settings_mic_level, "REAL-TIME MICROPHONE LEVEL", "NÍVEL DO MICROFONE EM TEMPO REAL", "NIVEL DE MICRÓFONO EN TIEMPO REAL", "ECHTZEIT-MIKROFONPEGEL", "NIVEAU DU MICROPHONE EN TEMPS RÉEL", "УРОВЕНЬ МИКРОФОНА В РЕАЛЬНОМ ВРЕМЕНИ", "リアルタイムマイク音量"),
    // Proxy test/alert strings (Network & Proxy tab + login alert). Machine
    // translations (es/de/fr/ru/ja) are placeholders pending native review.
    // System tray menu + tooltip. PT values keep the current wording; the
    // es/de/fr/ru/ja cells are machine translations pending native review.
    (tray_show, set_tray_show, "Open Litecord", "Exibir Litecord", "Abrir Litecord", "Litecord öffnen", "Ouvrir Litecord", "Открыть Litecord", "Litecord を開く"), // machine TODO
    (tray_quit, set_tray_quit, "Quit", "Sair", "Salir", "Beenden", "Quitter", "Выход", "終了"), // machine TODO
    (tray_tooltip, set_tray_tooltip, "Litecord — Ultra-Lightweight Discord Client", "Litecord - Discord Client Ultra-Leve", "Litecord: cliente de Discord ultraligero", "Litecord – ultraleichter Discord-Client", "Litecord — client Discord ultra-léger", "Litecord — сверхлёгкий клиент Discord", "Litecord — 超軽量 Discord クライアント"), // machine TODO
    // Option lists for the Video & Interface tab (built in Rust in
    // src/ui/helpers.rs set Video settings modal). BR values keep the current
    // wording; es/de/fr/ru/ja are machine translations pending review.
    (enc_auto_desc, set_enc_auto_desc, "Prefers available hardware GPU acceleration, falling back to CPU.", "Prioriza aceleração por hardware GPU disponível com fallback para CPU.", "Prioriza la aceleración de GPU disponible, con reserva a CPU.", "Bevorzugt verfügbare GPU-Beschleunigung, mit CPU-Fallback.", "Privilégie l'accélération GPU disponible, avec repli sur le CPU.", "Предпочитает доступное аппаратное ускорение GPU, при сбое — CPU.", "利用可能なGPUアクセラレーションを優先し、CPUへフォールバックします。"),
    (enc_nvenc_desc, set_enc_nvenc_desc, "Hardware accelerated encoding via NVIDIA GeForce / RTX GPUs.", "Codificação acelerada por hardware via GPU NVIDIA GeForce / RTX.", "Codificación acelerada por hardware en GPU NVIDIA GeForce/RTX.", "Hardware-beschleunigte Kodierung über NVIDIA GeForce/RTX-GPUs.", "Encodage accéléré par matériel sur GPU NVIDIA GeForce / RTX.", "Аппаратное кодирование на GPU NVIDIA GeForce / RTX.", "NVIDIA GeForce / RTX GPUによるハードウェアエンコード。"),
    (enc_ffmpeg_desc, set_enc_ffmpeg_desc, "GPU-accelerated pipeline via FFmpeg (VA-API, QSV, NVENC).", "Pipeline acelerado por GPU via FFmpeg (VA-API, QSV, NVENC).", "Pipeline acelerado por GPU vía FFmpeg (VA-API, QSV, NVENC).", "GPU-beschleunigte Pipeline über FFmpeg (VA-API, QSV, NVENC).", "Pipeline accéléré par GPU via FFmpeg (VA-API, QSV, NVENC).", "GPU-ускоренный конвейер через FFmpeg (VA-API, QSV, NVENC).", "FFmpeg (VA-API、QSV、NVENC) によるGPUアクセラレーションパイプライン。"),
    (enc_openh264_desc, set_enc_openh264_desc, "Universal multithreaded software encoding with SIMD optimizations.", "Codificação universal por software multithread com otimizações SIMD.", "Codificación por software universal multihilo con optimizaciones SIMD.", "Universelle multithreaded Software-Kodierung mit SIMD-Optimierung.", "Encodage logiciel universel multithread avec optimisations SIMD.", "Универсальное многопоточное программное кодирование с оптимизацией SIMD.", "マルチスレッドのソフトウェアエンコード（SIMD最適化）。"),
    (vcap_auto_desc, set_vcap_auto_desc, "Uses the system's native selector via XDG Desktop Portal and PipeWire.", "Utiliza o seletor nativo do sistema via XDG Desktop Portal e PipeWire.", "Usa el selector nativo del sistema vía XDG Desktop Portal y PipeWire.", "Nutzt den nativen Systemselektor über XDG Desktop Portal und PipeWire.", "Utilise le sélecteur natif du système via XDG Desktop Portal et PipeWire.", "Использует нативный селектор системы через XDG Desktop Portal и PipeWire.", "XDG Desktop PortalとPipeWireによるネイティブセレクターを使用。"),
    (vcap_portal_desc, set_vcap_portal_desc, "Modern low-latency screen capture compatible with Wayland and Flatpak.", "Captura de tela moderna de baixa latência compatível com Wayland e Flatpak.", "Captura de pantalla moderna de baja latencia compatible con Wayland y Flatpak.", "Moderne Latenz-arme Bildschirmaufnahme, kompatibel mit Wayland und Flatpak.", "Capture d'écran moderne à faible latence compatible Wayland et Flatpak.", "Современный захват экрана с низкой задержкой, совместим с Wayland и Flatpak.", "Wayland・Flatpak対応の低遅延スクリーンキャプチャ。"),
    (vcap_x11_desc, set_vcap_x11_desc, "Legacy direct screen capture on X11 sessions.", "Captura legada direta de tela em sessões gráficas X11.", "Captura de pantalla directa heredada en sesiones gráficas X11.", "Legacy direkte Bildschirmaufnahme in X11-Sitzungen.", "Capture d'écran directe héritée dans les sessions X11.", "Устаревший прямой захват экрана в X11-сессиях.", "X11セッション向けのレガシー直接キャプチャ。"),
    (vcap_printwin_desc, set_vcap_printwin_desc, "Direct window capture (foreground or background) via DWM.", "Captura direta de janelas em primeiro ou segundo plano via DWM.", "Captura directa de ventanas en primer o segundo plano vía DWM.", "Direkte Fensteraufnahme (Vorder-/Hintergrund) über DWM.", "Capture directe des fenêtres (avant ou arrière-plan) via DWM.", "Прямой захват окон (передний/задний план) через DWM.", "DWMによるウィンドウの直接キャプチャ（前面/背面）。"),
    (vcap_bitblt_desc, set_vcap_bitblt_desc, "Classic high-performance capture straight from the desktop framebuffer.", "Captura clássica de alto desempenho diretamente do framebuffer do desktop.", "Captura clásica de alto rendimiento directamente del framebuffer del escritorio.", "Klassische leistungsstarke Aufnahme direkt aus dem Desktop-Framebuffer.", "Capture classique performante depuis le framebuffer du bureau.", "Классический высокопроизводительный захват из фреймбуфера рабочего стола.", "デスクトップのフレームバッファから直接行う従来型高性能キャプチャ。"),
    (aloop_auto_desc, set_aloop_auto_desc, "Captures the default PulseAudio/PipeWire sink monitor via GStreamer pulsesrc.", "Captura monitor do sink padrão PulseAudio/PipeWire via GStreamer pulsesrc.", "Captura el monitor del sink PulseAudio/PipeWire vía GStreamer pulsesrc.", "Erfasst den PulseAudio/PipeWire-Sink-Monitor über GStreamer pulsesrc.", "Capture le moniteur du sink PulseAudio/PipeWire via GStreamer pulsesrc.", "Захватывает монитор PulseAudio/PipeWire через GStreamer pulsesrc.", "GStreamer pulsesrcによるPulseAudio/PipeWireシンクモニターのキャプチャ。"),
    (aloop_pulsesrc_desc, set_aloop_pulsesrc_desc, "Dedicated GStreamer pipeline connected to the PipeWire monitor sink.", "Pipeline GStreamer dedicado conectado ao sink monitor do PipeWire.", "Pipeline GStreamer dedicado conectado al sink monitor de PipeWire.", "Dedizierte GStreamer-Pipeline, verbunden mit dem PipeWire-Monitor-Sink.", "Pipeline GStreamer dédiée connectée au sink monitor de PipeWire.", "Отдельный конвейер GStreamer, подключённый к монитору PipeWire.", "PipeWireモニターシンクに接続する専用GStreamerパイプライン。"),
    (aloop_cpal_desc, set_aloop_cpal_desc, "Standard capture via the CPAL audio interface.", "Captura padrão via interface de áudio CPAL.", "Captura estándar vía la interfaz de audio CPAL.", "Standard-Aufnahme über die CPAL-Audioschnittstelle.", "Capture standard via l'interface audio CPAL.", "Стандартный захват через аудиоинтерфейс CPAL.", "CPALオーディオインターフェース経由の標準キャプチャ。"),
    (aloop_wasapi_desc, set_aloop_wasapi_desc, "Native capture that removes friends' voices and Litecord sounds from your stream.", "Captura nativa que remove vozes dos amigos e sons do Litecord da sua transmissão.", "Captura nativa que elimina las voces de amigos y sonidos de Litecord de tu direct en local.", "Nativaufnahme, das Stimmen von Freunden und Litecord-Sounds aus dem Stream entfernt.", "Capture native qui supprime les voix des amis et les sons de Litecord du direct.", "Нативный захват убирает голоса друзей и звуки Litecord из вашей трансляции.", "ネイティブキャプチャは、通話相手の声とLitecordのサウンドを配信に混ぜません。"),
    (aloop_wasapi2_desc, set_aloop_wasapi2_desc, "Process-isolated WASAPI capture (excludes Litecord audio from the stream).", "Captura com isolamento de processo WASAPI (exclui o Litecord do áudio da stream).", "Captura WASAPI con aislamiento de proceso (excluye el audio de Litecord del local en directo).", "Prozessisolierter WASAPI-Capture (Litecord-Audio bleibt draußen).", "Capture WASAPI isolée par processus (exclut l'audio de Litecord du direct).", "WASAPI с изоляцией процесса (Litecord не попадает в трансляцию).", "プロセス分離WASAPIキャプチャ（Litecordの音声は配信に含まれません）。"),
    // Proxy connection-mode option rows (Network & Proxy tab). Machine
    // translations pending native review.
    (proxy_off_name, set_proxy_off_name, "Disabled (direct connection)", "Desativado (Conexão Direta)", "Desactivado (conexión directa)", "Aus (direkte Verbindung)", "Désactivé (connexion directe)", "Отключено (прямое подключение)", "無効（直接接続）"), // machine TODO
    (proxy_off_desc, set_proxy_off_desc, "Direct connection to Discord's servers (default).", "Conexão direta aos servidores do Discord (Padrão).", "Conexión directa a los servidores de Discord (predeterminado).", "Direkte Verbindung zu den Discord-Servern (Standard).", "Connexion directe aux serveurs de Discord (par défaut).", "Прямое подключение к серверам Discord (по умолчанию).", "Discordサーバーへの直接接続（デフォルト）。"), // machine TODO
    (proxy_http_desc, set_proxy_http_desc, "HTTP CONNECT tunnel for restricted networks and proxies.", "Túnel HTTP CONNECT para redes restritas e proxies.", "Túnel HTTP CONNECT para redes restringidas y servidores proxy.", "HTTP-CONNECT-Tunnel für eingeschränkte Netze und Proxies.", "Tunnel HTTP CONNECT pour les réseaux restreints et les proxys.", "Туннель HTTP CONNECT для ограниченных сетей и прокси.", "制限ネットワークとプロキシ向けのHTTP CONNECTトンネル。"), // machine TODO
    (proxy_socks_desc, set_proxy_socks_desc, "SOCKS5 tunnel compatible with Tor, Shadowsocks and SSH.", "Túnel SOCKS5 compatível com Tor, Shadowsocks e SSH.", "Túnel SOCKS5 compatible con Tor, Shadowsocks y SSH.", "SOCKS5-Tunnel, kompatibel mit Tor, Shadowsocks und SSH.", "Tunnel SOCKS5 compatible Tor, Shadowsocks et SSH.", "Туннель SOCKS5 совместим с Tor, Shadowsocks и SSH.", "Tor・Shadowsocks・SSH対応のSOCKS5トンネル。"), // machine TODO
    (proxy_system_name, set_proxy_system_name, "System proxy", "Proxy do Sistema", "Proxy del sistema", "Systemproxy", "Proxy système", "Системный прокси", "システムプロキシ"), // machine TODO
    (proxy_system_desc, set_proxy_system_desc, "Detects the system's global proxy automatically.", "Detecta automaticamente o proxy global do sistema.", "Detecta automáticamente el proxy global del sistema.", "Erkennt den globalen Systemproxy automatisch.", "Détecte automatiquement le proxy global du système.", "Автоматически определяет системный прокси.", "システムのグローバルプロキシを自動検出します。"), // machine TODO
    // Windows-only capture/loopback wording (distinct from the unix arms) and
    // device-picker placeholders for empty enumerations.
    (vcap_gdi_auto_desc, set_vcap_gdi_auto_desc, "Tries PrintWindow for DirectX windows, using BitBlt GDI for the desktop and as fallback.", "Tenta PrintWindow para janelas DirectX e usa BitBlt GDI para desktop e fallback.", "Intenta PrintWindow para ventanas DirectX y usa BitBlt GDI para escritorio y reserva.", "Versucht PrintWindow für DirectX-Fenster und nutzt BitBlt GDI für den Desktop und als Fallback.", "Essaie PrintWindow pour les fenêtres DirectX et utilise BitBlt GDI pour le bureau en repli.", "Пробует PrintWindow для окон DirectX и использует BitBlt GDI для рабочего стола, как резерв.", "DirectXウィンドウにPrintWindowを試し、デスクトップにはBitBlt GDIをフォールバックとして使用します。"),
    (aloop_cpal_win_desc, set_aloop_cpal_win_desc, "Direct capture from the operating system's default output endpoint.", "Captura direta do endpoint de saída padrão do sistema operacional.", "Captura directa del endpoint de salida predeterminado del sistema operativo.", "Direkte Aufnahme vom Standard-Ausgabe-Endpoint des Betriebssystems.", "Capture directe de l'endpoint de sortie par défaut du système.", "Прямой захват с выходного endpoint системы по умолчанию.", "OSのデフォルト出力endpointからの直接キャプチャ。"),
    (device_default_input, set_device_default_input, "System Default Microphone", "Microfone Padrão do Sistema", "Micrófono predeterminado del sistema", "Systemstandardmikrofon", "Micro par défaut du système", "Микрофон по умолчанию", "システムの既定マイク"), // machine TODO
    (device_default_output, set_device_default_output, "System Default Speakers", "Alto-falantes Padrão do Sistema", "Altavoces predeterminados del sistema", "Systemstandardlautsprecher", "Haut-parleurs par défaut du système", "Динамики по умолчанию", "システムの既定スピーカー"), // machine TODO
    (proxy_test_success, set_proxy_test_success, "Proxy test successful! Latency: {}ms (Status: {})", "Conexão bem-sucedida! Latência: {}ms (Status: {})", "¡Conexión correcta! Latencia: {}ms (Estado: {})", "Verbindung erfolgreich! Latenz: {}ms (Status: {})", "Connexion réussie ! Latence : {}ms (Statut : {})", "Соединение успешно! Задержка: {} мс (Статус: {})", "接続成功！レイテンシ: {}ms (ステータス: {})"), // machine TODO
    (proxy_test_http_status, set_proxy_test_http_status, "Proxy responded in {}ms, but with HTTP status {}", "Proxy respondeu em {}ms, mas com status HTTP {}", "El proxy respondió en {}ms, pero con estado HTTP {}", "Proxy antwortete in {}ms, aber mit HTTP-Status {}", "Le proxy a répondu en {}ms, mais avec le statut HTTP {}", "Прокси ответил за {} мс, но со статусом HTTP {}", "プロキシは {}ms で応答しましたが、HTTPステータスは {} です"), // machine TODO
    (proxy_test_failed, set_proxy_test_failed, "Failed to connect via proxy: {}", "Falha ao conectar via Proxy: {}", "Fallo al conectar vía proxy: {}", "Verbindung über Proxy fehlgeschlagen: {}", "Échec de connexion via le proxy : {}", "Не удалось подключиться через прокси: {}", "プロキシ経由で接続できませんでした: {}"), // machine TODO
    (proxy_alert_system, set_proxy_alert_system, "System proxy connection failed! Click here or the gear at the top to open Settings and disable it.", "Falha de conexão via Proxy do Sistema! Clique aqui ou na engrenagem no topo para abrir as Configurações e desativá-lo.", "¡Fallo de conexión del proxy del sistema! Haz clic aquí o en el engranaje superior para abrir Ajustes y desactivarlo.", "Systemproxy-Verbindung fehlgeschlagen! Klicke hier oder auf das Zahnrad oben, um die Einstellungen zu öffnen und ihn zu deaktivieren.", "Échec de connexion du proxy système ! Clique ici ou sur la roue dentée en haut pour ouvrir les réglages et le désactiver.", "Ошибка подключения через системный прокси! Нажмите здесь или на шестерёнку сверху, чтобы открыть настройки и отключить его.", "システムプロキシの接続に失敗しました！ここまたは上部の歯車をクリックして設定を開き、無効にしてください。"), // machine TODO
    (proxy_alert_manual, set_proxy_alert_manual, "The proxy ({}: {}:{}) is unreachable or refusing connections! Click here or the gear at the top to open Settings and disable it.", "O Proxy ({}: {}:{}) está inacessível ou recusando conexão! Clique aqui ou na engrenagem no topo para abrir as Configurações e desativá-lo.", "El proxy ({}: {}:{}) es inalcanzable o rechaza conexiones! Haz clic aquí o en el engranaje superior para abrir Ajustes y desactivarlo.", "Der Proxy ({}: {}:{}) ist nicht erreichbar oder lehnt Verbindungen ab! Klicke hier oder auf das Zahnrad oben, um die Einstellungen zu öffnen und ihn zu deaktivieren.", "Le proxy ({} : {} : {}) est injoignable ou refuse les connexions ! Clique ici ou sur la roue dentée en haut pour ouvrir les réglages et le désactiver.", "Прокси ({}: {}:{}) недоступен или отклоняет подключения! Нажмите здесь или на шестерёнку сверху, чтобы открыть настройки и отключить его.", "プロキシ ({}: {}:{}) に到達できないか接続を拒否されています！ここまたは上部の歯車をクリックして設定を開き、無効にしてください。"), // machine TODO
    (settings_btn_test_start, set_settings_btn_test_start, "Test Microphone (\"Hear Yourself\")", "Testar Microfone (\"Se Ouvir\")", "Probar Micrófono (\"Escucharse\")", "Mikrofon testen (\"Sich hören\")", "Tester le Micro (\"S'entendre\")", "Проверить микрофон (\"Слышать себя\")", "マイクをテスト (自分の声を聞く)"),
    (settings_btn_test_stop, set_settings_btn_test_stop, "Stop Testing (Listening...)", "Parar Teste (Ouvindo...)", "Detener Prueba (Escuchando...)", "Test beenden (Hören...)", "Arrêter le Test (Écoute...)", "Остановить тест (Слушаем...)", "テスト停止 (聴取中...)"),
    (settings_done, set_settings_done, "Done", "Concluído", "Hecho", "Fertig", "Terminé", "Готово", "完了"),
    (keybind_mute_title, set_keybind_mute_title, "Toggle Mute", "Mutar / Desmutar Microfone", "Silenciar / Desilenciar Micrófono", "Stummschaltung umschalten", "Couper / Réactiver le micro", "Вкл/Выкл микрофон", "マイクミュート切替"),
    (keybind_mute_desc, set_keybind_mute_desc, "Global shortcut to mute or unmute your microphone during voice calls.", "Atalho global para mutar ou desmutar seu microfone durante chamadas.", "Atajo global para activar o desactivar tu micrófono en llamadas de voz.", "Globaler Shortcut zum Stummschalten oder Aktivieren des Mikrofons.", "Raccourci global pour couper ou réactiver votre micro en vocal.", "Глобальная клавиша для включения или выключения микрофона.", "通話中にマイクのミュート/ミュート解除を切り替えるグローバルショートカット。"),
    (keybind_recording_prompt, set_keybind_recording_prompt, "Press desired key combination...", "Pressione a combinação de teclas...", "Presiona la combinación de teclas...", "Tastenkombination drücken...", "Appuyez sur la combinaison de touches...", "Нажмите комбинацию клавиш...", "キーの組み合わせを押してください..."),
    (keybind_btn_record, set_keybind_btn_record, "Record Keybind", "Gravar Atalho", "Grabar Atajo", "Aufnehmen", "Enregistrer", "Записать клавишу", "キーを記録"),
    (keybind_btn_clear, set_keybind_btn_clear, "Disable", "Desativar", "Desactivar", "Deaktivieren", "Désactiver", "Отключить", "無効化"),
    (keybind_guide_title, set_keybind_guide_title, "Global Shortcuts", "Atalhos Globais", "Atajos Globales", "Globale Tastenkombinationen", "Raccourcis Globaux", "Глобальные клавиши", "グローバルショートカット"),
    (keybind_guide_desc, set_keybind_guide_desc, "Keybinds work seamlessly across all applications and games, even when Litecord is in background.", "Os atalhos funcionam em qualquer janela ou jogo em tela cheia, mesmo com o Litecord minimizado.", "Los atajos funcionan en cualquier ventana o juego a pantalla completa, incluso con Litecord minimizado.", "Shortcuts funktionieren auch in Vollbildspielen und im Hintergrund.", "Les raccourcis fonctionnent dans n'importe quel jeu ou application, même en arrière-plan.", "Горячие клавиши работают во всех играх и приложениях, даже когда окно свернуто.", "ゲーム中や最小化時でもバックグラウンドで動作します。"),
    (keybind_deafen_title, set_keybind_deafen_title, "Toggle Deafen", "Ensurdecer / Desensurdecer", "Ensordecer / Desensordecer", "Taubstellen umschalten", "Assourdir / Désassourdir", "Заглушить / Включить звук", "スピーカーミュート切替"),
    (keybind_deafen_desc, set_keybind_deafen_desc, "Global shortcut to deafen or undeafen all voice audio and microphone.", "Atalho global para ensurdecer ou desensurdecer todo áudio e microfone.", "Atajo global para ensordecer o reactivar todo el sonido y micrófono.", "Globaler Shortcut zum Deaktivieren von Audio und Mikrofon.", "Raccourci global pour couper tout le son entrant et votre micro.", "Глобальная клавиша для отключения всех звуков и микрофона.", "すべての音声とマイクを遮断するグローバルショートカット。"),
 }

#[cfg(test)]
const LANGUAGES: [&str; 7] = ["en", "pt", "es", "de", "fr", "ru", "ja"];

static CURRENT: AtomicU8 = AtomicU8::new(0);

fn index_of(lang: Language) -> u8 {
    match lang {
        Language::Portuguese => 1,
        Language::Spanish => 2,
        Language::German => 3,
        Language::French => 4,
        Language::Russian => 5,
        Language::Japanese => 6,
        _ => 0,
    }
}

/// Remembers the (already resolved) language so Rust-side messages follow the UI.
pub fn set_current(lang: Language) {
    CURRENT.store(index_of(lang), Ordering::Relaxed);
}

/// Language currently shown in the UI (English until the first `apply`).
pub fn current_language() -> Language {
    match CURRENT.load(Ordering::Relaxed) {
        1 => Language::Portuguese,
        2 => Language::Spanish,
        3 => Language::German,
        4 => Language::French,
        5 => Language::Russian,
        6 => Language::Japanese,
        _ => Language::English,
    }
}

/// Strings for the language currently shown in the UI.
pub fn ui_text() -> &'static UiStrings {
    match CURRENT.load(Ordering::Relaxed) {
        1 => &PT_UI,
        2 => &ES_UI,
        3 => &DE_UI,
        4 => &FR_UI,
        5 => &RU_UI,
        6 => &JA_UI,
        _ => &EN_UI,
    }
}

pub fn table_for(lang: Language) -> &'static UiStrings {
    match lang {
        Language::Portuguese => &PT_UI,
        Language::Spanish => &ES_UI,
        Language::German => &DE_UI,
        Language::French => &FR_UI,
        Language::Russian => &RU_UI,
        Language::Japanese => &JA_UI,
        _ => &EN_UI,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tr_props_are_served_by_ui_table() {
        let mut remaining: Vec<String> = Vec::new();
        let ui_root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/ui"));
        let mut stack = vec![ui_root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let p = entry.unwrap().path();
                if p.is_dir() {
                    stack.push(p);
                } else if p.extension().map(|e| e == "slint").unwrap_or(false) {
                    let text = std::fs::read_to_string(&p).unwrap();
                    for (i, l) in text.lines().enumerate() {
                        let t = l.trim();
                        if t.starts_with("in property <string> tr_")
                            || t.starts_with("in-out property <string> tr_")
                        {
                            let name = t
                                .trim_start_matches("in-out property <string> ")
                                .trim_start_matches("in property <string> ")
                                .split(&[':', ' '][..])
                                .next()
                                .unwrap()
                                .to_string();
                            // tr_stream_quality is a dynamic runtime value
                            // (resolution + fps), set from src/main.rs and not
                            // a translated string — exempt (plan Task 4).
                            if name != "tr_stream_quality" {
                                remaining.push(format!("{}:{}: {}", p.display(), i + 1, name));
                            }
                        }
                    }
                }
            }
        }
        assert!(
            remaining.is_empty(),
            "hand-wired tr_ props left: {remaining:?}"
        );
    }

    #[test]
    fn every_string_exists_in_every_language() {
        for (key, texts) in entries() {
            for (i, text) in texts.iter().enumerate() {
                assert!(!text.trim().is_empty(), "{key}: empty text for {}", LANGUAGES[i]);
            }
        }
    }



    #[test]
    fn no_duplicate_ui_keys() {
        // The Task-4 sweep introduced duplicate row keys once (cancel) — this
        // contract keeps the table single-keyed (branched Whitespace struct).
        let mut seen: Vec<&str> = Vec::new();
        for (key, _) in entries() {
            assert!(!seen.contains(&key), "duplicate ui_strings key: {key}");
            seen.push(key);
        }
    }



    #[test]
    fn device_default_rows_en_no_pt() {
        let keys = ["device_default_input", "device_default_output"];
        let entries = entries();
        for key in keys {
            let row = entries.iter().find(|(k, _)| *k == key)
                .unwrap_or_else(|| panic!("missing device row: {key}"));
            for (i, text) in row.1.iter().enumerate() {
                assert!(!text.trim().is_empty(), "{key}: empty text for {}", LANGUAGES[i]);
            }
            for ch in ['ã', 'õ', 'ç'] {
                assert!(!row.1[0].contains(ch), "{key}: PT-only char in EN fallback: {:?}", row.1[0]);
            }
        }
    }

    #[test]
    fn proxy_mode_option_rows_en_no_pt() {
        let keys = [
            "proxy_off_name", "proxy_off_desc", "proxy_http_desc",
            "proxy_socks_desc", "proxy_system_name", "proxy_system_desc",
        ];
        let entries = entries();
        for key in keys {
            let row = entries.iter().find(|(k, _)| *k == key)
                .unwrap_or_else(|| panic!("missing proxy-mode row: {key}"));
            for (i, text) in row.1.iter().enumerate() {
                assert!(!text.trim().is_empty(), "{key}: empty text for {}", LANGUAGES[i]);
            }
            for ch in ['ã', 'õ', 'ç'] {
                assert!(!row.1[0].contains(ch), "{key}: PT-only char in EN fallback: {:?}", row.1[0]);
            }
        }
    }

    #[test]
    fn video_interface_option_rows_en_no_pt() {
        // The encoder / capture / loopback option lists are built in Rust from
        // these rows (Video & Interface tab). EN is the fallback; PT is legacy.
        let keys = [
            "enc_auto_desc", "enc_nvenc_desc", "enc_ffmpeg_desc", "enc_openh264_desc",
            "vcap_auto_desc", "vcap_portal_desc", "vcap_x11_desc",
            "vcap_printwin_desc", "vcap_bitblt_desc", "vcap_gdi_auto_desc",
            "aloop_auto_desc", "aloop_pulsesrc_desc", "aloop_cpal_desc",
            "aloop_wasapi_desc", "aloop_wasapi2_desc", "aloop_cpal_win_desc",
        ];
        let entries = entries();
        for key in keys {
            let row = entries.iter().find(|(k, _)| *k == key)
                .unwrap_or_else(|| panic!("missing option row: {key}"));
            for (i, text) in row.1.iter().enumerate() {
                assert!(!text.trim().is_empty(), "{key}: empty text for {}", LANGUAGES[i]);
            }
            for ch in ['ã', 'õ', 'ç'] {
                assert!(!row.1[0].contains(ch), "{key}: PT-only char in EN fallback: {:?}", row.1[0]);
            }
        }
    }

    #[test]
    fn proxy_alert_strings_exist_for_every_language() {
        let keys = [
            "proxy_test_success",
            "proxy_test_http_status",
            "proxy_test_failed",
            "proxy_alert_system",
            "proxy_alert_manual",
        ];
        let entries = entries();
        for key in keys {
            assert!(
                entries.iter().any(|(k, _)| *k == key),
                "missing proxy row: {key}"
            );
        }
        for (key, texts) in entries.iter() {
            if !keys.contains(key) {
                continue;
            }
            for (i, text) in texts.iter().enumerate() {
                assert!(!text.trim().is_empty(), "{key}: empty text for {}", LANGUAGES[i]);
            }
            // English is the universal fallback: no Portuguese-only chars.
            for ch in ['ã', 'õ', 'ç'] {
                assert!(!texts[0].contains(ch), "{key}: PT-only char in EN fallback: {:?}", texts[0]);
            }
        }
    }

    #[test]
    fn english_table_has_no_portuguese_leftovers() {
        let accents = "áàâãéêíóôõúçÁÀÂÃÉÊÍÓÔÕÚÇ";
        for (key, texts) in entries() {
            assert!(
                !texts[0].chars().any(|c| accents.contains(c)),
                "{key}: English text still looks Portuguese: {}",
                texts[0]
            );
        }
    }

    #[test]
    fn placeholders_and_line_breaks_survive_translation() {
        for (key, texts) in entries() {
            for (i, text) in texts.iter().enumerate() {
                assert_eq!(
                    text.contains("{}"),
                    texts[0].contains("{}"),
                    "{key}: placeholder mismatch in {}",
                    LANGUAGES[i]
                );
                assert_eq!(
                    text.matches('\n').count(),
                    texts[0].matches('\n').count(),
                    "{key}: line breaks differ in {}",
                    LANGUAGES[i]
                );
            }
        }
    }

    #[test]
    fn translated_tables_are_really_translated() {
        // A table that silently fell back to English would pass the checks above.
        for (i, lang) in LANGUAGES.iter().enumerate().skip(1) {
            let same = entries().iter().filter(|(_, t)| t[i] == t[0]).count();
            assert!(same < 40, "{lang}: {same} strings are identical to English");
        }
    }

    #[test]
    fn current_language_selects_the_matching_table() {
        set_current(Language::German);
        assert_eq!(ui_text().not_connected, DE_UI.not_connected);
        set_current(Language::Japanese);
        assert_eq!(ui_text().not_connected, JA_UI.not_connected);
        set_current(Language::English);
        assert_eq!(ui_text().not_connected, EN_UI.not_connected);
    }
}
